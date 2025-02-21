use revm::{
    db::{CacheDB, EmptyDB, EmptyDBTyped},
    Evm,
};
use revm_primitives::{address, hex, AccountInfo, Address, Bytecode, TransactTo, B256, U256};
use std::{
    convert::Infallible,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use crate::{
    register_handler, store_path, tests::utils::TestEnvGuard, EXTCompileWorker, FetchedFnResult,
};

const FIBONACCI_CODE: &[u8] =
    &hex!("5f355f60015b8215601a578181019150909160019003916005565b9150505f5260205ff3");
const DEPLOYED_ADDRESS: Address = address!("0000000000000000000000000000000000001234");
type MockEVM = Evm<'static, Arc<EXTCompileWorker>, CacheDB<EmptyDBTyped<Infallible>>>;

fn create_ext_compile_worker(
    spin_lock: bool,
    primary: bool,
    timeout: Duration,
) -> EXTCompileWorker {
    let start_time = Instant::now();
    loop {
        match EXTCompileWorker::new(primary, 1, 3, 128) {
            Ok(worker) => {
                return worker;
            }
            Err(_) => {
                if !spin_lock {
                    panic!("Failed to create EXTCompileWorker");
                } else if start_time.elapsed() >= timeout {
                    panic!("Failed to create EXTCompileWorker within spinlock, make timeout error");
                }
                thread::sleep(Duration::from_secs(10));
            }
        }
    }
}

#[inline]
fn setup_evm(db_spin_lock: bool, primary: bool) -> (MockEVM, B256) {
    let timeout = Duration::from_secs(1800);
    let external = create_ext_compile_worker(db_spin_lock, primary, timeout);

    let ext_worker = Arc::new(external);
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

fn primary_worker_fn(spin_lock: bool) {
    let (mut evm, fib_hash) = setup_evm(spin_lock, true);
    let code_path = store_path().join(fib_hash.to_string());
    assert!(!code_path.exists(), "Ensure aot-compiled code doesn't exist");

    // First call - Execute by interpreter and AOT compile Fibonacci code
    evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    let mut result = evm.transact().unwrap();
    assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));

    // Poll for compilation completion with timeout
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(5);
    while !code_path.exists() {
        if start.elapsed() > timeout {
            panic!("Timeout waiting for AOT compilation");
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
    // Check AOT compilation was successful
    {
        let result: FetchedFnResult = evm.context.external.get_function(&fib_hash).unwrap();
        assert!(matches!(result, FetchedFnResult::Found(_)));
    }
    assert!(code_path.exists(), "Failed to AOT compile");
    // Second call - uses AOT-compiled machine code
    evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    result = evm.transact().unwrap();
    assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));

    // Check code loaded successfully in cache
    {
        let mut cache = evm.context.external.cache.write().unwrap();
        assert!(cache.get(&fib_hash).is_some(), "Failed to load in cache");
    }
    // Third call - uses cached code
    evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    result = evm.transact().unwrap();
    assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));
}

#[allow(dead_code)]
fn secondary_worker_fn() {
    let (mut evm, fib_hash) = setup_evm(false, false);
    evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    let mut result = evm.transact().unwrap();
    assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));

    // Check code loaded successfully in cache
    {
        let mut cache = evm.context.external.cache.write().unwrap();
        assert!(cache.get(&fib_hash).is_some(), "Failed to load in cache");
    }
    // uses cached code
    evm.context.evm.env.tx.transact_to = TransactTo::Call(DEPLOYED_ADDRESS);
    evm.context.evm.env.tx.data = U256::from(9).to_be_bytes_vec().into();
    result = evm.transact().unwrap();
    assert_eq!(U256::from_be_slice(result.result.output().unwrap()), U256::from(55));
}

#[test]
fn test_worker_suite() {
    test_worker_single_process();

    test_worker_multi_process();
}

fn test_worker_single_process() {
    let _g = TestEnvGuard::new();
    primary_worker_fn(true);
}

fn test_worker_multi_process() {
    let _g = TestEnvGuard::new();
    let mut processes = Vec::new();
    let primary_process = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("primary_worker_fn")
        .arg("true")
        .spawn()
        .expect("Failed to spawn child process");
    processes.push(primary_process);
    for _ in 0..10 {
        let secondary_process = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("secondary_worker_fn")
            .spawn()
            .expect("Failed to spawn child process");

        processes.push(secondary_process);
    }

    for process in processes {
        let result = process.wait_with_output().expect("Failed to wait on child");
        assert!(result.status.success(), "Child process failed");
    }
}
