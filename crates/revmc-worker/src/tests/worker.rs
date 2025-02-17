use alloy_primitives::address;
use revm::{
    db::{CacheDB, EmptyDB, EmptyDBTyped},
    Evm,
};
use revm_primitives::{hex, AccountInfo, Address, Bytecode, TransactTo, B256, U256};
use std::{convert::Infallible, sync::Arc, thread};

use crate::{
    register_handler, store_path, tests::utils::TestEnvGuard, EXTCompileWorker, FetchedFnResult,
};

const FIBONACCI_CODE: &[u8] =
    &hex!("5f355f60015b8215601a578181019150909160019003916005565b9150505f5260205ff3");
const DEPLOYED_ADDRESS: Address = address!("0000000000000000000000000000000000001234");
type MockEVM<'a> = Evm<'a, Arc<EXTCompileWorker>, CacheDB<EmptyDBTyped<Infallible>>>;

#[inline]
fn setup_evm() -> (MockEVM<'static>, B256) {
    let _g = TestEnvGuard::new();
    let ext_worker = Arc::new(EXTCompileWorker::new(1, 3, 128));
    let db = CacheDB::new(EmptyDB::new());
    let mut evm = revm::Evm::builder()
        .with_db(db)
        .with_external_context(ext_worker)
        .append_handler_register(register_handler)
        .build();
    let fib_bytecode = Bytecode::new_raw(FIBONACCI_CODE.into());
    let fib_hash = fib_bytecode.hash_slow();

    evm.db_mut().insert_account_info(
        DEPLOYED_ADDRESS,
        AccountInfo {
            code_hash: fib_hash,
            code: Some(Bytecode::new_raw(FIBONACCI_CODE.into())),
            ..Default::default()
        },
    );

    (evm, fib_hash)
}

#[test]
fn test_worker() {
    let (mut evm, fib_hash) = setup_evm();

    // First call - Execute by interpreter and JIT compile Fibonacci code
    evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    let mut result = evm.transact().unwrap();
    assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));

    // Wait for worker to JIT compile code
    thread::sleep(std::time::Duration::from_secs(2));
    // Check JIT compilation was successful
    {
        let result = evm.context.external.get_function(&fib_hash).unwrap();
        assert!(matches!(result, FetchedFnResult::Found(_)));
    }
    // let so_file_path = store_path().join(fib_hash.to_string()).join("a.so");
    // assert!(so_file_path.exists(), "Failed to JIT compile");
    // // Second call - uses jit-compiled machine code
    // evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    // evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    // result = evm.transact().unwrap();
    // assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));

    // // Check code loaded successfully in cache
    // {
    //     let mut cache = evm.context.external.cache.write().unwrap();
    //     assert!(cache.get(&fib_hash).is_some(), "Failed to load in cache");
    // }
    // // Third call - uses cached code
    // evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    // evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    // result = evm.transact().unwrap();
    // assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));
}
