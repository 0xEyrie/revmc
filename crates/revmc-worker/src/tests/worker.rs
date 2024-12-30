use alloy_primitives::{address, Keccak256, Uint};
use revm::{
    db::{CacheDB, EmptyDB},
    Evm,
};
use revm_primitives::{
    hex::FromHex, AccessList, AccessListItem, AccountInfo, BlockEnv, Bytecode, Bytes, TxEnv,
    TxKind, B256,
};
use std::{sync::Arc, thread};

use crate::{register_handler, EXTCompileWorker};

fn setup_test_cache(ext_worker: &Arc<EXTCompileWorker>, bytecode: &Bytecode) {
    let code_hash = bytecode.hash_slow();

    ext_worker.work(revm_primitives::SpecId::OSAKA, code_hash, bytecode.bytes());
    std::thread::sleep(std::time::Duration::from_secs(2));

    assert_eq!(ext_worker.get_function(code_hash).unwrap().is_some(), true);
}

#[inline]
fn fib_call_data() -> Bytes {
    let function_signature = "fibonacci(uint256)";
    let mut hasher = Keccak256::new();
    hasher.update(function_signature.as_bytes());
    let selector = hasher.finalize();
    let selector_bytes = &selector[0..4];

    let mut encoded_data = Vec::with_capacity(4 + 32);
    encoded_data.extend_from_slice(selector_bytes);

    let mut param = [0u8; 32];
    let value = 25u64;
    param[24..].copy_from_slice(&value.to_be_bytes());
    encoded_data.extend_from_slice(&param);

    Bytes::from(encoded_data)
}

#[test]
fn test_compiler_cache_retrieval() {
    let ext_worker = Arc::new(EXTCompileWorker::new(1, 3, 128));
    let bytecode = Bytecode::new_raw(Bytes::from_static(&[1, 2, 3]));

    setup_test_cache(&ext_worker, &bytecode);
}

#[test]
fn test_compiler_cache_load_access_list() {
    let ext_worker = Arc::new(EXTCompileWorker::new(1, 3, 128));

    // Create
    let caller_address = address!("e100713fc15400d1e94096a545879e7c647001e0");
    let fib_bin =
        "0x6080604052348015600e575f80fd5b5061020d8061001c5f395ff3fe608060405234801561000f575f80fd5b5060043610610029575f3560e01c806361047ff41461002d575b5f80fd5b610047600480360381019061004291906100f1565b61005d565b604051610054919061012b565b60405180910390f35b5f80820361006d575f90506100b5565b6001820361007e57600190506100b5565b61009360028361008e9190610171565b61005d565b6100a86001846100a39190610171565b61005d565b6100b291906101a4565b90505b919050565b5f80fd5b5f819050919050565b6100d0816100be565b81146100da575f80fd5b50565b5f813590506100eb816100c7565b92915050565b5f60208284031215610106576101056100ba565b5b5f610113848285016100dd565b91505092915050565b610125816100be565b82525050565b5f60208201905061013e5f83018461011c565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61017b826100be565b9150610186836100be565b925082820390508181111561019e5761019d610144565b5b92915050565b5f6101ae826100be565b91506101b9836100be565b92508282019050808211156101d1576101d0610144565b5b9291505056fea26469706673582212201e1e9b4f1c72cf916978622f2e9e7e62e962bf691679559ba242ea8e9bf74c6164736f6c63430008190033";
    let db = CacheDB::new(EmptyDB::new());

    let fib_bytecode = Bytecode::new_raw(fib_bin.into());
    let fib_hash = fib_bytecode.hash_slow();
    setup_test_cache(&ext_worker, &fib_bytecode);

    let mut evm = Evm::builder()
        .with_external_context(ext_worker.clone())
        .with_db(db)
        .append_handler_register(register_handler)
        .build();

    evm.context.evm.inner.env.block = BlockEnv::default();
    evm.context.evm.inner.env.block.gas_limit = Uint::from(50_000_000);

    evm.context.evm.inner.env.tx = TxEnv::default();
    evm.context.evm.inner.env.tx.caller = caller_address;
    evm.context.evm.inner.env.tx.gas_limit = 50_000_000;
    evm.context.evm.inner.env.tx.transact_to = TxKind::Create;
    evm.context.evm.inner.env.tx.data = Bytes::from_hex(fib_bin).unwrap();

    let deployed_address = address!("ec30481c768e48d34ea8fc2bebcfeaddeba6bfa4");

    // Manually insert account info for deployed contract
    // Not necessary in practice
    evm.db_mut().insert_account_info(
        deployed_address,
        AccountInfo {
            code_hash: fib_bytecode.hash_slow(),
            code: Some(fib_bytecode),
            ..Default::default()
        },
    );

    // Call
    let access_list = AccessList(vec![AccessListItem {
        address: deployed_address,
        storage_keys: vec![B256::ZERO],
    }]);

    // Cache upfront
    if let Err(err) = ext_worker.preload_cache(vec![fib_hash]) {
        println!("While cache_load_access_list: {:#?}", err);
    }
    thread::sleep(std::time::Duration::from_secs(2));

    evm.context.evm.inner.env.tx.transact_to = TxKind::Call(deployed_address);
    evm.context.evm.inner.env.tx.data = fib_call_data();
    evm.context.evm.inner.env.tx.access_list = access_list.to_vec();

    if let Err(err) = evm.transact() {
        println!("While evm transact: {:#?}", err)
    };
}
