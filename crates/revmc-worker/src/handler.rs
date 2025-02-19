use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};

use revm::{handler::register::EvmHandler, Database};

use crate::{EXTCompileWorker, FetchedFnResult};

// Register handler for external context to support background compile worker in node runtime
pub fn register_handler<DB: Database + 'static>(
    handler: &mut EvmHandler<'_, Arc<EXTCompileWorker>, DB>,
) {
    let prev = handler.execution.execute_frame.clone();
    handler.execution.execute_frame = Arc::new(move |frame, memory, tables, context| {
        let interpreter = frame.interpreter_mut();
        let code_hash = interpreter.contract.hash.unwrap_or_default();

        match context.external.get_function(&code_hash) {
            Ok(FetchedFnResult::NotFound) => {
                let spec_id = context.evm.inner.spec_id();
                let bytecode = context.evm.db.code_by_hash(code_hash).unwrap_or_default();
                let _res = context.external.spwan(spec_id, code_hash, bytecode.original_bytes());
                #[cfg(feature = "tracing")]
                if let Err(err) = res {
                    tracing::error!("Worker failed: with bytecode hash {}: {:#?}", code_hash, err);
                }

                prev(frame, memory, tables, context)
            }

            Ok(FetchedFnResult::Found(f)) => {
                let res = catch_unwind(AssertUnwindSafe(|| unsafe {
                    f.call_with_interpreter_and_memory(interpreter, memory, context)
                }));

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
