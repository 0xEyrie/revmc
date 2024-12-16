use super::path::aot_store_path;
use crate::error::CompilerError;

use revm_primitives::{Bytes, SpecId, B256};
use revmc::{EvmCompiler, OptimizationLevel};
use revmc_llvm::EvmLlvmBackend;

#[derive(Debug)]
pub(crate) struct AotRuntime {
    pub cfg: AotCfg,
}

impl AotRuntime {
    pub(crate) fn new(cfg: AotCfg) -> Self {
        Self { cfg }
    }

    pub(crate) fn compile(
        &self,
        code_hash: B256,
        bytecode: Bytes,
        spec_id: SpecId,
    ) -> Result<(), CompilerError> {
        let _ = color_eyre::install();

        let context = revmc_llvm::inkwell::context::Context::create();
        let backend = EvmLlvmBackend::new_for_target(
            &context,
            self.cfg.aot,
            self.cfg.opt_level,
            &revmc_backend::Target::Native,
        )
        .map_err(|err| CompilerError::BackendInit { err: err.to_string() })?;

        let mut compiler = EvmCompiler::new(backend);

        let out_dir = aot_store_path();
        std::fs::create_dir_all(&out_dir)
            .map_err(|err| CompilerError::FileIO { err: err.to_string() })?;

        compiler.set_dump_to(Some(out_dir.clone()));
        compiler.gas_metering(self.cfg.no_gas);

        unsafe {
            compiler.stack_bound_checks(self.cfg.no_len_checks);
        }

        let name = code_hash.to_string();
        compiler.frame_pointers(true);
        compiler.debug_assertions(self.cfg.debug_assertions);
        compiler.set_module_name(name.to_string());
        compiler.validate_eof(true);

        compiler.inspect_stack_length(true);
        let _f_id = compiler
            .translate(&name, &bytecode, spec_id)
            .map_err(|err| CompilerError::BytecodeTranslation { err: err.to_string() })?;

        let module_out_dir = out_dir.join(&name);
        std::fs::create_dir_all(&module_out_dir)
            .map_err(|err| CompilerError::FileIO { err: err.to_string() })?;

        // Compile.
        let obj = module_out_dir.join("a.o");
        compiler
            .write_object_to_file(&obj)
            .map_err(|err| CompilerError::FileIO { err: err.to_string() })?;

        // Link.
        let so_path = module_out_dir.join("a.so");
        let linker = revmc::Linker::new();
        linker
            .link(&so_path, [obj.to_str().unwrap()])
            .map_err(|err| CompilerError::Link { err: err.to_string() })?;

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct AotCfg {
    pub aot: bool,
    pub opt_level: OptimizationLevel,
    pub no_gas: bool,
    pub no_len_checks: bool,
    pub debug_assertions: bool,
}

impl Default for AotCfg {
    fn default() -> Self {
        Self {
            aot: true,
            opt_level: OptimizationLevel::Aggressive,
            no_gas: true,
            no_len_checks: true,
            debug_assertions: true,
        }
    }
}
