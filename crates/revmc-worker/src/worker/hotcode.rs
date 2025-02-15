use revm_primitives::B256;
use rocksdb::{ DB, Options, Error };
use tokio::time;
use std::{ sync::Mutex, thread };

use super::db_path;

/// Embedded Database to support below features
/// Use RocksDB to support multi-process and multi-thread
/// 1. Count the call of contracts to find hot contract code
/// 2. Save the path of machincode result to load
#[derive(Debug)]
pub(crate) struct HotCodeCounter(Mutex<DB>);

impl HotCodeCounter {
    pub(crate) fn new(worker_pool_size: usize) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.increase_parallelism(worker_pool_size as i32);
        opts.set_max_background_jobs(worker_pool_size as i32);
        opts.set_max_write_buffer_number(worker_pool_size as i32);
        opts.set_max_open_files(1000);
        let db_path = db_path();
        let path = db_path.to_str().unwrap();

        let mut db: Option<DB> = None;
        while db.is_none() {
            match DB::open(&opts, path) {
                Ok(database) => {
                    db = Some(database);
                }
                Err(e) => {
                    thread::sleep(time::Duration::from_secs(2));
                }
            }
        }

        Self(Mutex::new(db.unwrap()))
    }

    pub(crate) fn load_hot_call_count(&self, code_hash: B256) -> Result<u64, Error> {
        let db = self.0.lock().unwrap();
        match db.get(code_hash) {
            Ok(Some(count)) => {
                let count: [u8; 8] = count
                    .as_slice()
                    .try_into()
                    .expect("slice with incorrect length");
                Ok(u64::from_be_bytes(count))
            }
            Ok(None) => Ok(0),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn write_hot_call_count(&self, code_hash: B256, value: u64) -> Result<(), Error> {
        let value = value.to_be_bytes();
        let db = self.0.lock().unwrap();
        db.put(code_hash, value)
    }
}
