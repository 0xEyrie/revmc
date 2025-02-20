use std::{ fmt::{ self, Debug }, num::NonZeroUsize, sync::{ Arc, RwLock, TryLockError } };

use crate::{
    error::Error,
    module_name,
    worker::{ store_path, AotCompileWorkerPool, HotCodeCounter },
};
use libloading::{ Library, Symbol };
use lru::LruCache;
use revm_primitives::{ Bytes, SpecId, B256 };
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

#[derive(Debug)]
pub struct EvmCompilerFnTuple((EvmCompilerFn, Arc<Library>));
/// Compiler Worker as an external context.
///
/// External function fetching is optimized by using an LRU Cache.
/// In many cases, a contract that is called will likely be called again,
/// so the cache helps reduce library loading cost.
pub struct EXTCompileWorker {
    worker_pool: AotCompileWorkerPool,
    pub cache: RwLock<LruCache<B256, EvmCompilerFnTuple>>,
}

impl EXTCompileWorker {
    pub fn new(
        primary: bool,
        threshold: u64,
        worker_pool_size: usize,
        cache_size_words: usize
    ) -> Result<Self, Error> {
        let hot_code_counter = HotCodeCounter::new(primary, worker_pool_size)?;
        let worker_pool = AotCompileWorkerPool::new(threshold, hot_code_counter, worker_pool_size);

        Ok(Self {
            worker_pool,
            cache: RwLock::new(LruCache::new(NonZeroUsize::new(cache_size_words).unwrap())),
        })
    }

    /// Fetches the compiled function from disk, if exists
    pub fn get_function(&self, code_hash: &B256) -> Result<FetchedFnResult, Error> {
        if code_hash.is_zero() {
            return Ok(FetchedFnResult::NotFound);
        }
        // Write locks are required for reading from LRU Cache
        {
            let mut acq = true;

            let cache = match self.cache.try_write() {
                Ok(c) => Some(c),
                Err(err) =>
                    match err {
                        /* in this case, read from file instead of cache */
                        TryLockError::WouldBlock => {
                            acq = false;
                            None
                        }
                        TryLockError::Poisoned(err) => Some(err.into_inner()),
                    }
            };

            if acq {
                if let Some(t) = cache.unwrap().get(code_hash) {
                    return Ok(FetchedFnResult::Found(t.0.0));
                }
            }
        }

        let name = module_name();
        let so = store_path().join(code_hash.to_string()).join("a.so");
        if so.try_exists().unwrap_or(false) {
            {
                let lib = Arc::new((unsafe { Library::new(so) })?);
                let f: Symbol<'_, revmc::EvmCompilerFn> = unsafe { lib.get(name.as_bytes())? };

                let tuple = EvmCompilerFnTuple((*f, lib.clone()));
                let mut cache = self.cache
                    .write()
                    .map_err(|err| Error::RwLockPoison { err: err.to_string() })?;
                cache.put(*code_hash, tuple);

                return Ok(FetchedFnResult::Found(*f));
            }
        }
        Ok(FetchedFnResult::NotFound)
    }

    /// Spwan AOT compile the byecode referred by code_hash
    pub fn spwan(&self, spec_id: SpecId, code_hash: B256, bytecode: Bytes) -> Result<(), Error> {
        self.worker_pool.spwan(spec_id, code_hash, bytecode);
        Ok(())
    }
}
