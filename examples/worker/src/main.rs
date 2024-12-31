//! Simple compiler worker example.

use std::{sync::Arc, thread};

use common::{FIBONACCI_CODE, FIBONACCI_HASH};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        address, AccessList, AccessListItem, AccountInfo, Bytecode, TransactTo, B256, U256,
    },
};
use revmc_worker::{register_handler, EXTCompileWorker};

mod common;

fn main() {
    let ext_worker = Arc::new(EXTCompileWorker::new(1, 3, 128));

    let db = CacheDB::new(EmptyDB::new());
    let mut evm = revm::Evm::builder()
        .with_db(db)
        .with_external_context(ext_worker.clone())
        .append_handler_register(register_handler)
        .build();

    let fibonacci_address = address!("0000000000000000000000000000000000001234");
    evm.db_mut().insert_account_info(
        fibonacci_address,
        AccountInfo {
            code_hash: FIBONACCI_HASH.into(),
            code: Some(Bytecode::new_raw(FIBONACCI_CODE.into())),
            ..Default::default()
        },
    );

    let access_list = AccessList(vec![AccessListItem {
        address: fibonacci_address,
        storage_keys: vec![B256::ZERO],
    }]);

    ext_worker.preload_cache(vec![B256::from(FIBONACCI_HASH)]).unwrap();
    thread::sleep(std::time::Duration::from_secs(2));

    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
    evm.context.evm.env.tx.data = U256::from(10).to_be_bytes_vec().into();
    evm.context.evm.inner.env.tx.access_list = access_list.to_vec();

    let result = evm.transact().unwrap();
    println!("fib(10) = {}", U256::from_be_slice(result.result.output().unwrap()));
}
