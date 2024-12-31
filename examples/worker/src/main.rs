//! Simple compiler worker example.

use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        address, AccessList, AccessListItem, AccountInfo, Bytecode, TransactTo, B256, U256,
    },
};
use revmc_worker::{register_handler, EXTCompileWorker};
use std::thread;

use common::FIBONACCI_CODE;

mod common;

/// First call executes the transaction and compiles into embedded db
/// embedded db: ~/.aotstore/db, ~/.aotstore/output
/// It is crucial to reset the embedded db for reproducing the same steps
/// Otherwise, both calls will utilize cached ExternalFn
///
/// Second call loads the ExternalFn from embedded db to cache
/// and executes transaction with it
fn main() {
    let ext_worker = EXTCompileWorker::new(1, 3, 128);

    let db = CacheDB::new(EmptyDB::new());
    let mut evm = revm::Evm::builder()
        .with_db(db)
        .with_external_context(ext_worker.clone())
        .append_handler_register(register_handler)
        .build();

    let fibonacci_address = address!("0000000000000000000000000000000000001234");

    let fib_bytecode = Bytecode::new_raw(FIBONACCI_CODE.into());
    let fib_hash = fib_bytecode.hash_slow();

    evm.db_mut().insert_account_info(
        fibonacci_address,
        AccountInfo {
            code_hash: fib_hash.into(),
            code: Some(Bytecode::new_raw(FIBONACCI_CODE.into())),
            ..Default::default()
        },
    );

    let access_list = AccessList(vec![AccessListItem {
        address: fibonacci_address,
        storage_keys: vec![B256::ZERO],
    }]);

    // First call - compiles ExternalFn
    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    evm.context.evm.inner.env.tx.access_list = access_list.to_vec();
    thread::sleep(std::time::Duration::from_secs(2));

    ext_worker.preload_cache(vec![B256::from(fib_hash)]).unwrap();

    // Second call - uses cached ExternalFn
    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    evm.context.evm.inner.env.tx.access_list = access_list.to_vec();

    let result = evm.transact().unwrap();
    println!("fib(10) = {}", U256::from_be_slice(result.result.output().unwrap()));
}
