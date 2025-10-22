//! Simple compiler worker example.

use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{address, hex, AccountInfo, Bytecode, TransactTo, U256},
};
use revmc_worker::{register_handler, store_path, EXTCompileWorker};
use std::{fs, sync::Arc, thread};

pub const FIBONACCI_CODE: &[u8] =
    &hex!("5f355f60015b8215601a578181019150909160019003916005565b9150505f5260205ff3");

/// Performance comparison example:
/// 1. Interpretation (first call, cold)
/// 2. AOT compilation (background)
/// 3. Load from disk (second call after compilation)
/// 4. Cache hit (third call, hot)
fn main() {
    // Clean up previous compilation artifacts
    let output_path = store_path();
    if output_path.exists() {
        println!("=== Cleaning up previous artifacts ===");
        fs::remove_dir_all(&output_path).ok();
        println!("Removed: {}\n", output_path.display());
    }

    println!("=== Initializing EXTCompiler ===");
    let init_start = std::time::Instant::now();
    let ext_worker = Arc::new(EXTCompileWorker::new(1, 3, 128).unwrap());
    println!("Initialization took: {:?}\n", init_start.elapsed());
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
            code_hash: fib_hash,
            code: Some(Bytecode::new_raw(FIBONACCI_CODE.into())),
            ..Default::default()
        },
    );

    // Test Case 1: Interpretation (cold start)
    println!("=== Case 1: Interpretation (first execution) ===");
    let start = std::time::Instant::now();
    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
    evm.context.evm.env.tx.data = U256::from(99).to_be_bytes_vec().into();
    let mut result = evm.transact().unwrap();
    let elapsed = start.elapsed();
    println!("fib(100) = {}", U256::from_be_slice(result.result.output().unwrap()));
    println!("Execution time: {:?}", elapsed);
    println!("Gas used: {}\n", result.result.gas_used());

    // Wait for AOT compilation to complete
    println!("=== Waiting for AOT compilation... ===");
    let so_path = output_path.join(fib_hash.to_string()).join("a.so");
    let wait_start = std::time::Instant::now();
    let mut compiled = false;
    for i in 0..100 {
        if so_path.exists() {
            println!("✓ Compilation completed in {:?}\n", wait_start.elapsed());
            compiled = true;
            break;
        }
        if i % 10 == 0 && i > 0 {
            println!("  Still waiting... ({:?})", wait_start.elapsed());
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
    if !compiled {
        println!("✗ Compilation not completed after 10s\n");
    }

    // Test Case 2: Load from disk (first AOT execution)
    println!("=== Case 2: AOT compiled (load from disk) ===");
    let start = std::time::Instant::now();
    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
    evm.context.evm.env.tx.data = U256::from(99).to_be_bytes_vec().into();
    result = evm.transact().unwrap();
    let elapsed = start.elapsed();
    println!("fib(100) = {}", U256::from_be_slice(result.result.output().unwrap()));
    println!("Execution time: {:?}", elapsed);
    println!("Gas used: {}\n", result.result.gas_used());

    // Test Case 3: Cache hit (hot path)
    println!("=== Case 3: AOT compiled (from cache) ===");
    let start = std::time::Instant::now();
    evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
    evm.context.evm.env.tx.data = U256::from(99).to_be_bytes_vec().into();
    result = evm.transact().unwrap();
    let elapsed = start.elapsed();
    println!("fib(100) = {}", U256::from_be_slice(result.result.output().unwrap()));
    println!("Execution time: {:?}", elapsed);
    println!("Gas used: {}\n", result.result.gas_used());

    // Test Case 4: Multiple runs to measure consistency
    println!("=== Case 4: Consistency test (10 runs) ===");
    let mut times = Vec::new();
    for _ in 0..10 {
        let start = std::time::Instant::now();
        evm.context.evm.env.tx.transact_to = TransactTo::Call(fibonacci_address);
        evm.context.evm.env.tx.data = U256::from(99).to_be_bytes_vec().into();
        evm.transact().unwrap();
        times.push(start.elapsed());
    }
    let avg = times.iter().sum::<std::time::Duration>() / times.len() as u32;
    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();
    println!("Average: {:?}", avg);
    println!("Min: {:?}", min);
    println!("Max: {:?}", max);
}
