use revm_primitives::B256;
use rocksdb::{Options, DB};
use std::sync::Mutex;

use crate::error::Error;

use super::{db_path, sc_db_path};

/// Embedded Database to support the following features:
/// - Use RocksDB to support multi-threading (not multi-processing)
/// - Count the number of contract calls to identify hot contract code
/// - Save the compiled shared object (so) file after successful compilation
#[derive(Debug)]
pub(crate) struct HotCodeCounter {
    pub primary: bool,
    counter: Mutex<DB>,
}

impl HotCodeCounter {
    pub(crate) fn new(primary: bool, worker_pool_size: usize) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.increase_parallelism(worker_pool_size as i32);
        opts.set_max_background_jobs(worker_pool_size as i32);
        opts.set_max_write_buffer_number(worker_pool_size as i32);
        opts.set_max_open_files(worker_pool_size as i32);
        let db_path = db_path();
        let primary_path = db_path.to_str().unwrap();
        let db = if primary {
            DB::open(&opts, primary_path)?
        } else {
            let sc_db_path = sc_db_path();
            let secondary_path = sc_db_path.to_str().unwrap();
            DB::open_as_secondary(&opts, primary_path, secondary_path)?
        };

        Ok(Self { primary, counter: Mutex::new(db) })
    }

    pub(crate) fn load_hot_call_count(&self, code_hash: B256) -> Result<u64, Error> {
        let db = self.counter.lock().unwrap();
        match db.get(code_hash) {
            Ok(Some(count)) => {
                let count: [u8; 8] =
                    count.as_slice().try_into().expect("slice with incorrect length");
                Ok(u64::from_be_bytes(count))
            }
            Ok(None) => Ok(0),
            Err(err) => Err(Error::Database(err)),
        }
    }

    pub(crate) fn write_hot_call_count(&self, code_hash: B256, value: u64) -> Result<(), Error> {
        let value = value.to_be_bytes();
        let db = self.counter.lock().unwrap();
        db.put(code_hash, value)?;
        Ok(())
    }
}
