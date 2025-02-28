use crate::error::Error;

use super::{
    compiler::{AotCompiler, AotConfig},
    get_runtime,
    hotcode::HotCodeCounter,
};

use revm_primitives::B256;
use revmc::primitives::{Bytes, SpecId};
use std::{
    fmt::{self, Debug},
    sync::Arc,
};
use tokio::sync::{Mutex, Semaphore};

/// A worker responsible for compiling bytecode in machine code.
#[derive(Debug)]
pub struct AotCompileWorkerPool {
    pub threshold: u64,
    semaphore: Arc<Semaphore>,
    inner: Arc<WorkerPoolInner>,
}

impl Debug for WorkerPoolInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkerPoolInner")
            .field("hot_code_counter", &self.hot_code_counter)
            .field("aot_compiler", &self.aot_compiler)
            .finish()
    }
}

struct WorkerPoolInner {
    hot_code_counter: HotCodeCounter,
    aot_compiler: Mutex<AotCompiler>,
}

impl AotCompileWorkerPool {
    /// Creates a new `CompileWorkerPool`.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold for the number of times a bytecode must be seen before it is
    ///   compiled.
    /// * `hot_code_counter` - A reference-counted, thread-safe handle to count call of contract
    /// * `max_concurrent_tasks` - The maximum number of concurrent aot compiling tasks allowed.
    pub(crate) fn new(
        threshold: u64,
        hot_code_counter: HotCodeCounter,
        max_concurrent_tasks: usize,
    ) -> Self {
        Self {
            threshold,
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
            inner: Arc::new(WorkerPoolInner {
                hot_code_counter,
                aot_compiler: Mutex::new(AotCompiler::new(AotConfig::default())),
            }),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_with_config(
        threshold: u64,
        hot_code_counter: HotCodeCounter,
        max_concurrent_tasks: usize,
        config: AotConfig,
    ) -> Self {
        Self {
            threshold,
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
            inner: Arc::new(WorkerPoolInner {
                hot_code_counter,
                aot_compiler: Mutex::new(AotCompiler::new(config)),
            }),
        }
    }

    /// Spawns a compilation task for the given bytecode with the specified specId.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The specification ID for the EVM.
    /// * `code_hash` - The hash of the bytecode to be compiled.
    /// * `bytecode` - The bytecode to be compiled.
    ///
    /// # Returns
    ///
    /// A `JoinHandle` to the spawned task, which resolves to a `Result` indicating success or
    /// failure.
    pub(crate) fn spwan(
        &self,
        spec_id: SpecId,
        code_hash: B256,
        bytecode: Bytes,
    ) -> Result<(), Error> {
        let threshold = self.threshold;
        let semaphore = self.semaphore.clone();
        let inner = self.inner.clone();
        if !inner.hot_code_counter.primary {
            return Ok(());
        }
        let runtime = get_runtime();
        runtime.spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            // Check if the bytecode is all zeros
            if code_hash.is_zero() {
                return Ok(());
            }
            let counter = &inner.hot_code_counter;

            // Read the current count of the bytecode hash from the embedded database
            let count = counter.load_hot_call_count(code_hash)?;
            let new_count = count + 1;
            if new_count == threshold {
                let aot_compiler = inner.aot_compiler.lock().await;
                match aot_compiler.compile(code_hash, bytecode, spec_id).await {
                    Ok(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::info!("Compiled bytecode hash: {:#x}", code_hash);
                    }
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::error!(
                            "Failed to compile bytecode hash: {:#x}, error: {:#?}",
                            code_hash,
                            err
                        );
                        // Skip updating the count of contract calls to ensure that
                        // "so file doesn't exist" means the count is less than the threshold.
                        return Err(err);
                    }
                }
            }
            // Only write the new count to the database after compiling successfully
            counter.write_hot_call_count(code_hash, new_count)
        });

        Ok(())
    }
}
