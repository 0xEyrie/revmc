//! Simple compiler worker example.

use revm::{
    db::{ CacheDB, EmptyDB },
    primitives::{ address, hex, AccountInfo, Bytecode, TransactTo, U256 },
    Evm,
};
use revmc_worker::{ register_handler, EXTCompileWorker };
use std::{
    ops::{ Add, Div },
    sync::Arc,
    thread,
    time::Duration,
};
use std::time::Instant;

pub const FIBONACCI_CODE: &[u8] = &hex!(
    "5f355f60015b8215601a578181019150909160019003916005565b9150505f5260205ff3"
);

fn fib(num: u64, iter: u32) {
    let db = CacheDB::new(EmptyDB::new());
    let mut evm = revm::Evm::builder().with_db(db).build();
    execute_fib::<()>(num, iter, &mut evm);
}

fn fib_with_aot_compiler(num: u64, iter: u32) {
    let ext_worker = Arc::new(EXTCompileWorker::new(1, 3, 128));

    let db = CacheDB::new(EmptyDB::new());
    let mut evm = revm::Evm
        ::builder()
        .with_db(db)
        .with_external_context(ext_worker.clone())
        .append_handler_register(register_handler)
        .build();

    execute_fib::<Arc<EXTCompileWorker>>(num, iter, &mut evm);
}

fn execute_fib<T>(
    num: u64,
    iter: u32,
    evm: &mut Evm<'_, T, CacheDB<revm::db::EmptyDBTyped<std::convert::Infallible>>>
) {
    let fibonacci_address = address!("0000000000000000000000000000000000001234");

    let fib_bytecode = Bytecode::new_raw(FIBONACCI_CODE.into());
    let fib_hash = fib_bytecode.hash_slow();

    evm.db_mut().insert_account_info(fibonacci_address, AccountInfo {
        code_hash: fib_hash,
        code: Some(Bytecode::new_raw(FIBONACCI_CODE.into())),
        ..Default::default()
    });

    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
    evm.context.evm.env.tx.data = U256::from(num).to_be_bytes_vec().into();
    evm.transact().unwrap();

    thread::sleep(Duration::from_secs(1));
    let mut duration_sum = Duration::new(0, 0);
    for _ in 0..iter {
        let timer = Instant::now();
        let _result = evm.transact().unwrap();
        let duration = timer.elapsed();
        duration_sum = duration_sum.add(duration);
    }
    println!("Execution Mean time of fib({:?}): {:?}", num, duration_sum.div(iter));
}

/// First call executes the transaction and compiles into embedded db
/// embedded db: ~/.aotstore/db, ~/.aotstore/output
/// It is crucial to reset the embedded db and do 'cargo clean' for reproducing the same steps
/// Otherwise, both calls will utilize cached ExternalFn or unexpected behavior will happen
///
/// Second call loads the ExternalFn from embedded db to cache
/// and executes transaction with it
fn main() {
    println!("Call Fibonachi Contract");
    fib(1, 100);
    fib(10, 100);
    fib(100, 100);
    fib(5000, 100);
    fib(50000, 100);

    println!("Call Fibonachi Contract With AOT Compiler");
    fib_with_aot_compiler(1, 100);
    fib_with_aot_compiler(10, 100);
    fib_with_aot_compiler(100, 100);
    fib_with_aot_compiler(5000, 100);
    fib_with_aot_compiler(50000, 100);
}
