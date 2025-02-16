use super::{
    hotcode::HotCodeCounter,
    runtime::{get_runtime, JitConfig, JitRuntime},
};
use alloy_primitives::B256;
use revmc::primitives::{Bytes, SpecId};
use rocksdb::Error;
use std::sync::Arc;
use tokio::{sync::Semaphore, task::JoinHandle};

/// A worker responsible for compiling bytecode in machine code.
#[derive(Debug)]
pub struct CompileWorker {
    pub threshold: u64,
    hot_code_counter: Arc<HotCodeCounter>,
    jit_runtime: Arc<JitRuntime>,
    semaphore: Arc<Semaphore>,
}

impl CompileWorker {
    /// Creates a new `CompileWorker`.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold for the number of times a bytecode must be seen before it is
    ///   compiled.
    /// * `hot_code_counter` - A reference-counted, thread-safe handle to count call of
    /// * `max_concurrent_tasks` - The maximum number of concurrent jit tasks allowed.
    pub(crate) fn new(
        threshold: u64,
        hot_code_counter: HotCodeCounter,
        max_concurrent_tasks: usize,
    ) -> Self {
        Self {
            threshold,
            hot_code_counter: Arc::new(hot_code_counter),
            jit_runtime: Arc::new(JitRuntime::new(JitConfig::default())),
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
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
    ) -> JoinHandle<Result<(), Error>> {
        let threshold = self.threshold;
        let semaphore = self.semaphore.clone();
        let hotcode_counter = self.hot_code_counter.clone();
        let jit_rt = self.jit_runtime.clone();
        let runtime = get_runtime();
        runtime.spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            // Check if the bytecode is all zeros
            if code_hash.is_zero() {
                return Ok(());
            }
            // Read the current count of the bytecode hash from the embedded database
            let count = hotcode_counter.load_hot_call_count(code_hash).unwrap();
            let new_count = count + 1;
            // Check if the bytecode should be compiled
            if new_count == threshold {
                // Compile the bytecode
                match jit_rt.compile(code_hash, bytecode, spec_id) {
                    Ok(_) => {
                        tracing::info!("Compiled bytecode hash: {:#x}", code_hash);
                    }
                    Err(err) => {
                        tracing::error!(
                            "Failed to compile bytecode hash: {:#x}, error: {:#?}",
                            code_hash,
                            err
                        );
                        return Ok(());
                    }
                }
            }
            // Only write the new count to the database after compiling successfully
            hotcode_counter.write_hot_call_count(code_hash, new_count)
        })
    }
}
