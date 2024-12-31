use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};

use revm::{handler::register::EvmHandler, Database};

use crate::EXTCompileWorker;

// Register handler for external context to support background compile worker in node runtime
//
// Placeholders for handling unexpected behaviors are left to be placed by developers
pub fn register_handler<DB: Database + 'static>(
    handler: &mut EvmHandler<'_, Arc<EXTCompileWorker>, DB>,
) {
    let prev = handler.execution.execute_frame.clone();
    handler.execution.execute_frame = Arc::new(move |frame, memory, tables, context| {
        let interpreter = frame.interpreter_mut();
        let code_hash = interpreter.contract.hash.unwrap_or_default();
        let spec_id = context.evm.inner.spec_id();

        match context.external.get_function(code_hash) {
            Ok(None) => {
                let bytecode = context.evm.db.code_by_hash(code_hash).unwrap_or_default();

                if let Err(_err) =
                    context.external.work(spec_id, code_hash, bytecode.original_bytes())
                {
                };
                prev(frame, memory, tables, context)
            }

            Ok(Some(f)) => {
                let res = catch_unwind(AssertUnwindSafe(|| unsafe {
                    f.call_with_interpreter_and_memory(interpreter, memory, context)
                }));

                if let Err(_err) = &res {}
                Ok(res.unwrap())
            }

            Err(_err) => prev(frame, memory, tables, context),
        }
    });
}
