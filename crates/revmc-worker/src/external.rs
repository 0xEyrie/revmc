use std::{
    fmt::{self, Debug},
    num::NonZeroUsize,
    sync::{RwLock, TryLockError},
};

use crate::{
    error::ExtError,
    module_name,
    worker::{store_path, AotCompileWorkerPool, HotCodeCounter},
};
use alloy_primitives::B256;
use lru::LruCache;
use revm_primitives::{Bytes, SpecId};
use revmc::EvmCompilerFn;

#[derive(PartialEq, Debug)]
pub enum FetchedFnResult {
    Found(EvmCompilerFn),
    NotFound,
}

impl Debug for EXTCompileWorker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EXTCompileWorker")
            .field("worker_pool", &self.worker_pool)
            .field("cache", &self.cache)
            .finish()
    }
}

/// Compiler Worker as an external context.
///
/// External function fetching is optimized by using an LRU Cache.
/// In many cases, a contract that is called will likely be called again,
/// so the cache helps reduce library loading cost.
pub struct EXTCompileWorker {
    worker_pool: AotCompileWorkerPool,
    pub cache: RwLock<LruCache<B256, (EvmCompilerFn, libloading::Library)>>,
}

impl EXTCompileWorker {
    pub fn new(threshold: u64, worker_pool_size: usize, cache_size_words: usize) -> Self {
        let hot_code_counter = HotCodeCounter::new(worker_pool_size);
        let compile_worker =
            AotCompileWorkerPool::new(threshold, hot_code_counter, worker_pool_size);

        Self {
            worker_pool: compile_worker,
            cache: RwLock::new(LruCache::new(NonZeroUsize::new(cache_size_words).unwrap())),
        }
    }

    /// Fetches the compiled function from disk, if exists
    pub fn get_function(&self, code_hash: &B256) -> Result<FetchedFnResult, ExtError> {
        if code_hash.is_zero() {
            return Ok(FetchedFnResult::NotFound);
        }
        // Write locks are required for reading from LRU Cache
        {
            let mut acq = true;

            let cache = match self.cache.try_write() {
                Ok(c) => Some(c),
                Err(err) => match err {
                    /* in this case, read from file instead of cache */
                    TryLockError::WouldBlock => {
                        acq = false;
                        None
                    }
                    TryLockError::Poisoned(err) => Some(err.into_inner()),
                },
            };

            if acq {
                if let Some((f, _)) = cache.unwrap().get(code_hash) {
                    return Ok(FetchedFnResult::Found(*f));
                }
            }
        }
        let name = module_name();
        let so = store_path().join(code_hash.to_string()).join("a.so");
        if so.try_exists().unwrap_or(false) {
            {
                let lib = (unsafe { libloading::Library::new(&so) })
                    .map_err(|err| ExtError::LibLoadingError { err: err.to_string() })?;

                let f = unsafe {
                    *lib.get(name.as_bytes())
                        .map_err(|err| ExtError::GetSymbolError { err: err.to_string() })?
                };

                let mut cache = self
                    .cache
                    .write()
                    .map_err(|err| ExtError::RwLockPoison { err: err.to_string() })?;
                cache.put(*code_hash, (f, lib));

                return Ok(FetchedFnResult::Found(f));
            }
        }
        Ok(FetchedFnResult::NotFound)
    }

    /// Starts compile routine aot compile the code referred by code_hash
    pub fn spwan(&self, spec_id: SpecId, code_hash: B256, bytecode: Bytes) -> Result<(), ExtError> {
        self.worker_pool.spwan(spec_id, code_hash, bytecode);

        Ok(())
    }
}
