use libloading::Error as LibLoadingError;
use rocksdb::Error as DbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] DbError),

    #[error("Backend init error, err: {err}")]
    BackendInit { err: String },

    #[error("File I/O error, err: {err}")]
    FileIO { err: String },

    #[error("Bytecode translation error, err: {err}")]
    BytecodeTranslation { err: String },

    #[error("Link error, err: {err}")]
    Link { err: String },

    #[error("Lib loading error: {0}")]
    LibLoading(#[from] LibLoadingError),

    #[error("Get symbol error: {err}")]
    GetSymbol { err: String },

    #[error("RwLock poison error: {err}")]
    RwLockPoison { err: String },
}
