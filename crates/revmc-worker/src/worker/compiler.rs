use std::sync::{Arc, RwLock};

use alloy_primitives::B256;
use revmc::primitives::SpecId;

use super::{
    aot::{AotCfg, AotRuntime},
    runtime::get_runtime,
    sleddb::SledDB,
};

fn ivec_to_u64(ivec: &sled::IVec) -> Option<u64> {
    ivec.as_ref().try_into().ok().map(u64::from_be_bytes)
}

/// A worker responsible for compiling bytecode in machine code.
#[derive(Debug)]
pub struct CompileWorker {
    pub threshold: u64,
    sled_db: Arc<RwLock<SledDB<B256>>>,
    aot_runtime: Arc<AotRuntime>,
}

impl CompileWorker {
    /// Creates a new `CompileWorker`.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold for the number of times a bytecode must be seen before it is compiled.
    /// * `sled_db` - A reference-counted, thread-safe handle to the sled database.
    pub(crate) fn new(threshold: u64, sled_db: Arc<RwLock<SledDB<B256>>>) -> Self {
        Self { threshold, sled_db, aot_runtime: Arc::new(AotRuntime::new(AotCfg::default())) }
    }

    // Work the given bytecode with the specific specId.
    pub(crate) fn work(
        &mut self,
        spec_id: SpecId,
        code_hash: B256,
        bytecode: revm::primitives::Bytes,
    ) {
        let count = {
            let db_read = match self.sled_db.read() {
                Ok(lock) => lock,
                Err(poisoned) => poisoned.into_inner(),
            };
            let count_bytes = db_read.get(code_hash).unwrap_or(None);
            count_bytes.and_then(|v| ivec_to_u64(&v)).unwrap_or(0)
        };
        // 1. read codeahash count from embedded db
        let new_count = count + 1;

        let sled_db = Arc::clone(&self.sled_db);
        let aot_runtime = self.aot_runtime.clone();
        let threshold = self.threshold;

        let runtime = get_runtime();
        runtime.spawn(async move {
            // 2. check if bytecode is all zeros
            if code_hash.iter().all(|&b| b == 0) {
                return;
            }
            // 3. check condition of compile
            if new_count == threshold {
                // Compile the bytecode
                let label = code_hash.to_string().leak();
                match aot_runtime.compile(label, bytecode.as_ref(), spec_id) {
                    Ok(_) => {
                        tracing::info!("Compiled: bytecode hash {}", code_hash);
                    }
                    Err(err) => {
                        tracing::error!("Compile: with bytecode hash {} {:#?}", code_hash, err);
                        return;
                    }
                }
            }
            // 4. new count db commit
            {
                let db_write = match sled_db.write() {
                    Ok(lock) => lock,
                    Err(poisoned) => poisoned.into_inner(),
                };
                db_write.put(code_hash, &new_count.to_be_bytes()).unwrap();
            }
        });
    }
}
