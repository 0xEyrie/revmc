use revm_primitives::B256;
use rocksdb::{Options, WriteBatch, DB};
use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use crate::error::Error;

const DEFAULT_BUFFER_SIZE_LIMIT: usize = 1000;

/// Tracks accumulated gas usage per contract to identify hot code.
///
/// Uses RocksDB for thread-safe persistence with in-memory buffering.
/// When accumulated gas exceeds a threshold, the contract becomes eligible for AOT compilation.
#[derive(Debug)]
pub(crate) struct HotCodeDetector {
    db: Mutex<DB>,
    buffer: Mutex<HashMap<B256, u64>>,
    buffer_size_limit: usize,
}

impl HotCodeDetector {
    /// Creates a new HotCodeDetector with RocksDB backend.
    ///
    /// # Arguments
    /// * `db_path` - Directory path for RocksDB storage
    /// * `worker_pool_size` - Parallelism hint for RocksDB optimization
    pub(crate) fn new(db_path: PathBuf, worker_pool_size: usize) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.increase_parallelism(worker_pool_size as i32);
        opts.set_max_background_jobs(worker_pool_size as i32);
        opts.set_max_write_buffer_number(worker_pool_size as i32);

        let db = DB::open(&opts, db_path)?;

        Ok(Self {
            db: Mutex::new(db),
            buffer: Mutex::new(HashMap::new()),
            buffer_size_limit: DEFAULT_BUFFER_SIZE_LIMIT,
        })
    }

    /// Loads accumulated gas for a contract.
    ///
    /// Checks buffer first, then falls back to DB.
    /// Returns 0 if the contract hasn't been tracked yet.
    pub(crate) fn load_accumulated_gas(&self, code_hash: B256) -> Result<u64, Error> {
        // Check buffer first
        {
            let buffer = self.buffer.lock().unwrap();
            if let Some(&gas) = buffer.get(&code_hash) {
                return Ok(gas);
            }
        }

        // Fall back to DB
        let db = self.db.lock().unwrap();
        match db.get(code_hash) {
            Ok(Some(bytes)) => {
                let gas_bytes: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .expect("invalid gas value length");
                Ok(u64::from_be_bytes(gas_bytes))
            }
            Ok(None) => Ok(0),
            Err(err) => Err(Error::Database(err)),
        }
    }

    /// Stores accumulated gas for a contract in memory buffer.
    ///
    /// Automatically flushes to DB when buffer exceeds size limit.
    pub(crate) fn write_accumulated_gas(&self, code_hash: B256, gas: u64) -> Result<(), Error> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.insert(code_hash, gas);

        // Flush if buffer is full
        if buffer.len() >= self.buffer_size_limit {
            drop(buffer); // Release lock before flushing
            self.flush()?;
        }

        Ok(())
    }

    /// Flushes memory buffer to RocksDB.
    fn flush(&self) -> Result<(), Error> {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            return Ok(());
        }

        let db = self.db.lock().unwrap();
        let mut batch = WriteBatch::default();

        for (code_hash, gas) in buffer.drain() {
            batch.put(code_hash, gas.to_be_bytes());
        }

        db.write(batch)?;
        Ok(())
    }
}
