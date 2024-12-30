use std::{
    fmt::Debug,
    num::NonZeroUsize,
    sync::{Arc, RwLock, TryLockError},
};

use crate::{
    error::ExtError,
    worker::{aot_store_path, CompileWorker, SledDB},
};
use alloy_primitives::B256;
use lru::LruCache;
use once_cell::sync::OnceCell;
use revm_primitives::{Bytes, SpecId};
use revmc::EvmCompilerFn;

pub(crate) static SLED_DB: OnceCell<Arc<RwLock<SledDB<B256>>>> = OnceCell::new();

/// Compiler Worker as an external context.
///
/// External function fetching is optimized by using an LRU Cache.
/// In many cases, a contract that is called will likely be called again,
/// so the cache helps reduce disk I/O cost.
#[derive(Debug)]
pub struct EXTCompileWorker {
    compile_worker: Arc<CompileWorker>,
    pub cache: RwLock<LruCache<B256, (EvmCompilerFn, libloading::Library)>>,
}

impl EXTCompileWorker {
    pub fn new(threshold: u64, max_concurrent_tasks: usize, cache_size_words: usize) -> Self {
        let sled_db = SLED_DB.get_or_init(|| Arc::new(RwLock::new(SledDB::init())));
        let compiler = CompileWorker::new(threshold, Arc::clone(sled_db), max_concurrent_tasks);

        Self {
            compile_worker: Arc::new(compiler),
            cache: RwLock::new(LruCache::new(NonZeroUsize::new(cache_size_words).unwrap())),
        }
    }

    /// Does not utilize EXT for Address Zero
    /// When parallel compilation is in progress for the same code_hash,
    /// it resumes without utilizing the cached ExternalFn
    pub fn get_function(&self, code_hash: B256) -> Result<Option<EvmCompilerFn>, ExtError> {
        if code_hash.is_zero() {
            return Ok(None);
        }

        {
            let mut write_lock = match self.cache.try_write() {
                Ok(g) => g,
                Err(err) => match err {
                    // Compile in progress, resume without EXT
                    TryLockError::WouldBlock => {
                        return Ok(None);
                    }
                    TryLockError::Poisoned(err) => {
                        return Err(ExtError::RwLockPoison { err: err.to_string() });
                    }
                },
            };

            if let Some((f, _)) = write_lock.get(&code_hash) {
                return Ok(Some(*f));
            }
        }

        let so_file = aot_store_path().join(code_hash.to_string()).join("a.so");
        let exist: bool = so_file.try_exists().unwrap_or(false);
        if exist {
            {
                let lib = (unsafe { libloading::Library::new(&so_file) })
                    .map_err(|err| ExtError::LibLoadingError { err: err.to_string() })?;

                let f: EvmCompilerFn = unsafe {
                    *lib.get(code_hash.to_string().as_ref())
                        .map_err(|err| ExtError::GetSymbolError { err: err.to_string() })?
                };

                // The function holds a reference to the library, so dropping the library will cause an error.
                // Therefore, the library must also be stored in the cache.
                let mut write_lock = match self.cache.write() {
                    Ok(g) => g,
                    Err(err) => return Err(ExtError::RwLockPoison { err: err.to_string() }),
                };
                write_lock.put(code_hash, (f, lib));

                if let Some((f, _)) = write_lock.get(&code_hash) {
                    return Ok(Some(*f));
                } else {
                    return Err(ExtError::LruCacheGetError);
                };
            }
        }

        Ok(None)
    }

    pub fn work(&self, spec_id: SpecId, code_hash: B256, bytecode: Bytes) -> Result<(), ExtError> {
        self.compile_worker.work(spec_id, code_hash, bytecode);

        Ok(())
    }

    /// Preloads cache upfront for the specified code hashes
    /// Intended to improve runtime performance by
    /// reducing the overhead for fetching ExternalFn
    pub fn preload_cache(&self, code_hashes: Vec<B256>) -> Result<(), ExtError> {
        for code_hash in code_hashes.into_iter() {
            self.get_function(code_hash)?;
        }

        Ok(())
    }
}
