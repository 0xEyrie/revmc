use std::{ panic::{ catch_unwind, AssertUnwindSafe }, sync::Arc };

use revm::{ handler::register::EvmHandler, Database };

use crate::{ EXTCompileWorker, FetchedFnResult };

// Register handler for external context to support background compile worker in node runtime
pub fn register_compiler<DB: Database + 'static>(
    handler: &mut EvmHandler<'_, Arc<EXTCompileWorker>, DB>
) {
    let prev = handler.execution.execute_frame.clone();
    handler.execution.execute_frame = Arc::new(move |frame, memory, tables, context| {
        let interpreter = frame.interpreter_mut();
        let code_hash = interpreter.contract.hash.unwrap_or_default();

        match context.external.get_function(&code_hash) {
            Ok(FetchedFnResult::NotFound) => {
                // Capture gas before execution
                let gas_before = frame.interpreter().gas.remaining();

                // Execute via interpreter
                let result = prev(frame, memory, tables, context);

                // Track gas usage and potentially queue compilation
                if result.is_err() || code_hash.is_zero() {
                    return result;
                }
                let gas_after = frame.interpreter().gas.remaining();
                let gas_used = gas_before.saturating_sub(gas_after);
                let spec_id = context.evm.inner.spec_id();
                let bytecode = context.evm.db.code_by_hash(code_hash).unwrap_or_default();

                let _res = context.external.spwan(
                    spec_id,
                    code_hash,
                    bytecode.original_bytes(),
                    gas_used
                );
                #[cfg(feature = "tracing")]
                if let Err(err) = _res {
                    tracing::error!("Failed to queue compilation for {:#x}: {:#?}", code_hash, err);
                }

                result
            }

            Ok(FetchedFnResult::Found(f)) => {
                let res = catch_unwind(
                    AssertUnwindSafe(|| unsafe {
                        f.call_with_interpreter_and_memory(interpreter, memory, context)
                    })
                );

                #[cfg(feature = "tracing")]
                if let Err(err) = &res {
                    tracing::error!(
                        "AOT function call error: with bytecode hash {} {:#?}",
                        code_hash,
                        err
                    );
                }

                Ok(res.unwrap())
            }

            Err(_err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Error occurred in handler: {:?}", _err);
                prev(frame, memory, tables, context)
            }
        }
    });
}
