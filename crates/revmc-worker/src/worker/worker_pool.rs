use crate::error::Error;

use super::{
    compiler::{AotCompiler, AotConfig},
    get_runtime,
    hotcode::HotCodeDetector,
};

use revm_primitives::B256;
use revmc::primitives::{Bytes, SpecId};
use std::{
    fmt::{self, Debug},
    path::PathBuf,
    sync::{atomic::{AtomicUsize, Ordering}, Arc},
};
use tokio::sync::{Mutex, Semaphore};

/// AOT compilation worker pool.
///
/// Manages concurrent compilation tasks and tracks contract "hotness" via
/// accumulated gas usage. Contracts exceeding `gas_threshold` are compiled.
pub struct AotCompileWorkerPool {
    /// Gas threshold for triggering AOT compilation
    pub gas_threshold: u64,
    /// Maximum pending tasks allowed
    max_pending: usize,
    /// Current pending task count
    pending_count: Arc<AtomicUsize>,
    /// Limits concurrent compilation tasks
    semaphore: Arc<Semaphore>,
    /// Shared state across workers
    inner: Arc<WorkerPoolInner>,
}

impl Debug for AotCompileWorkerPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AotCompileWorkerPool")
            .field("gas_threshold", &self.gas_threshold)
            .field("max_pending", &self.max_pending)
            .field("pending_count", &self.pending_count.load(Ordering::Relaxed))
            .finish()
    }
}

impl Debug for WorkerPoolInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkerPoolInner")
            .field("hot_code_detector", &self.hot_code_detector)
            .field("aot_compiler", &self.aot_compiler)
            .finish()
    }
}

struct WorkerPoolInner {
    store_path: PathBuf,
    hot_code_detector: HotCodeDetector,
    aot_compiler: Mutex<AotCompiler>,
}

impl AotCompileWorkerPool {
    /// Creates a worker pool with default AOT compiler configuration.
    ///
    /// # Arguments
    /// * `store_path` - Directory path to store compiled .so files
    /// * `gas_threshold` - Accumulated gas threshold for triggering compilation
    /// * `hot_code_detector` - Tracks gas usage per contract
    /// * `max_concurrent_tasks` - Max parallel compilation jobs
    pub(crate) fn new(
        store_path: PathBuf,
        gas_threshold: u64,
        hot_code_detector: HotCodeDetector,
        max_concurrent_tasks: usize,
    ) -> Self {
        Self {
            gas_threshold,
            max_pending: max_concurrent_tasks * 2,
            pending_count: Arc::new(AtomicUsize::new(0)),
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
            inner: Arc::new(WorkerPoolInner {
                store_path,
                hot_code_detector,
                aot_compiler: Mutex::new(AotCompiler::new(AotConfig::default())),
            }),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_with_config(
        store_path: PathBuf,
        gas_threshold: u64,
        hot_code_detector: HotCodeDetector,
        max_concurrent_tasks: usize,
        config: AotConfig,
    ) -> Self {
        Self {
            gas_threshold,
            max_pending: max_concurrent_tasks * 2,
            pending_count: Arc::new(AtomicUsize::new(0)),
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
            inner: Arc::new(WorkerPoolInner {
                store_path,
                hot_code_detector,
                aot_compiler: Mutex::new(AotCompiler::new(config)),
            }),
        }
    }

    /// Spawns AOT compilation task when gas threshold is reached.
    ///
    /// Tracks accumulated gas usage per contract. Once the threshold is exceeded,
    /// triggers asynchronous compilation.
    ///
    /// # Arguments
    /// * `spec_id` - EVM specification version
    /// * `code_hash` - Contract bytecode hash
    /// * `bytecode` - Contract bytecode
    /// * `gas_used` - Gas consumed by this execution
    pub(crate) fn spwan(
        &self,
        spec_id: SpecId,
        code_hash: B256,
        bytecode: Bytes,
        gas_used: u64,
    ) -> Result<(), Error> {
        if code_hash.is_zero() {
            return Ok(());
        }

        // Check if queue is full
        let pending = self.pending_count.load(Ordering::Relaxed);
        if pending >= self.max_pending {
            #[cfg(feature = "tracing")]
            tracing::debug!("Compile queue full ({}/{}), dropping task for {:#x}", pending, self.max_pending, code_hash);
            return Ok(());
        }

        // Increment pending count
        self.pending_count.fetch_add(1, Ordering::Relaxed);

        let gas_threshold = self.gas_threshold;
        let semaphore = self.semaphore.clone();
        let inner = self.inner.clone();
        let pending_count = self.pending_count.clone();
        let runtime = get_runtime();

        runtime.spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let detector = &inner.hot_code_detector;

            // Load current accumulated gas
            let accumulated_gas = detector.load_accumulated_gas(code_hash)?;
            let new_accumulated_gas = accumulated_gas.saturating_add(gas_used);

            // Trigger compilation when crossing threshold
            if accumulated_gas < gas_threshold && new_accumulated_gas >= gas_threshold {
                let aot_compiler = inner.aot_compiler.lock().await;
                match aot_compiler.compile(code_hash, bytecode, spec_id).await {
                    Ok(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::info!(
                            "Compiled contract {:#x} (gas: {} >= {})",
                            code_hash,
                            new_accumulated_gas,
                            gas_threshold
                        );
                    }
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::error!("Compilation failed for {:#x}: {:#?}", code_hash, err);
                        // Don't update gas counter on compilation failure
                        pending_count.fetch_sub(1, Ordering::Relaxed);
                        return Err(err);
                    }
                }
            }

            // Update accumulated gas
            let result = detector.write_accumulated_gas(code_hash, new_accumulated_gas);

            // Decrement pending count
            pending_count.fetch_sub(1, Ordering::Relaxed);

            result
        });

        Ok(())
    }
}
