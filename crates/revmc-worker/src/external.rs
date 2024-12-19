use std::sync::{Arc, RwLock};

use crate::{
    error::ExtError,
    worker::{aot_store_path, CompileWorker, SledDB},
};
use alloy_primitives::B256;
use once_cell::sync::OnceCell;
use revm_primitives::SpecId;
use revmc::EvmCompilerFn;

pub(crate) static SLED_DB: OnceCell<Arc<RwLock<SledDB<B256>>>> = OnceCell::new();

#[derive(Debug)]
pub struct EXTCompileWorker {
    compile_worker: Box<CompileWorker>,
}

impl EXTCompileWorker {
    pub fn new(threshold: u64, max_concurrent_tasks: usize) -> Self {
        let sled_db = SLED_DB.get_or_init(|| Arc::new(RwLock::new(SledDB::init())));
        let compiler = CompileWorker::new(threshold, Arc::clone(sled_db), max_concurrent_tasks);
        Self { compile_worker: Box::new(compiler) }
    }

    pub fn get_function(
        &self,
        code_hash: B256,
    ) -> Result<Option<(EvmCompilerFn, libloading::Library)>, ExtError> {
        if code_hash.is_zero() {
            return Ok(None);
        }
        let label = code_hash.to_string();
        let so_file = aot_store_path().join(label).join("a.so");
        let exist: bool = so_file.try_exists().unwrap_or(false);
        if exist {
            let lib;
            let f = {
                lib = (unsafe { libloading::Library::new(&so_file) })
                    .map_err(|err| ExtError::LibLoadingError { err: err.to_string() })?;
                let f: libloading::Symbol<'_, revmc::EvmCompilerFn> = unsafe {
                    lib.get(code_hash.to_string().as_ref())
                        .map_err(|err| ExtError::GetSymbolError { err: err.to_string() })?
                };
                *f
            };

            return Ok(Some((f, lib)));
        }

        Ok(None)
    }

    pub fn work(&mut self, spec_id: SpecId, code_hash: B256, bytecode: revm::primitives::Bytes) {
        self.compile_worker.work(spec_id, code_hash, bytecode);
    }
}
