use std::fs;

use crate::error::Error;

use super::{env::store_path, module_name};

use revm_primitives::{Bytes, SpecId, B256};
use revmc::{EvmCompiler, OptimizationLevel};
use revmc_llvm::EvmLlvmBackend;

/// AOT configuration flags
/// Extra configurations are available in revmc-cli
#[derive(Debug)]
pub(crate) struct AotConfig {
    pub opt_level: OptimizationLevel,
    pub no_gas: bool,
    pub no_len_checks: bool,
}

impl Default for AotConfig {
    fn default() -> Self {
        Self { opt_level: OptimizationLevel::Aggressive, no_gas: false, no_len_checks: false }
    }
}

#[derive(Debug)]
pub(crate) struct AotCompiler {
    pub cfg: AotConfig,
}

impl AotCompiler {
    pub(crate) fn new(cfg: AotConfig) -> Self {
        Self { cfg }
    }

    /// Compile in Ahead of Time
    pub(crate) async fn compile(
        &self,
        code_hash: B256,
        bytecode: Bytes,
        spec_id: SpecId,
    ) -> Result<(), Error> {
        let context = revmc_llvm::inkwell::context::Context::create();
        let backend = EvmLlvmBackend::new_for_target(
            &context,
            true,
            self.cfg.opt_level,
            &revmc_backend::Target::Native,
        )
        .map_err(|err| Error::BackendInit { err: err.to_string() })?;
        let mut compiler = EvmCompiler::new(backend);
        compiler.gas_metering(self.cfg.no_gas);
        unsafe {
            compiler.stack_bound_checks(self.cfg.no_len_checks);
        }
        let name = module_name();
        compiler.set_module_name(&name);
        compiler.inspect_stack_length(true);

        // Compile.
        let _f_id = compiler
            .translate(&name, &bytecode, spec_id)
            .map_err(|err| Error::BytecodeTranslation { err: err.to_string() })?;

        let out_dir = store_path();
        let module_out_dir = out_dir.join(code_hash.to_string());
        fs::create_dir_all(&module_out_dir)
            .map_err(|err| Error::FileIO { err: err.to_string() })?;
        // Write object file
        let obj = module_out_dir.join("a.o");
        compiler
            .write_object_to_file(&obj)
            .map_err(|err| Error::FileIO { err: err.to_string() })?;
        // Link.
        let so_path = module_out_dir.join("a.so");
        let linker = revmc::Linker::new();
        linker
            .link(&so_path, [obj.to_str().unwrap()])
            .map_err(|err| Error::Link { err: err.to_string() })?;

        // Delete object files to reduce storage usage
        fs::remove_file(&obj).map_err(|err| Error::FileIO { err: err.to_string() })?;
        Ok(())
    }
}
