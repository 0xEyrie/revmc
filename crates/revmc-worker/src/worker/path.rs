use std::path::PathBuf;

/// Returns the default path
#[inline]
fn default_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_path = std::env::var("AOTSTORE_PATH").unwrap_or_else(|_| ".aotstore".to_string());
    PathBuf::from(home_dir).join(config_path)
}

/// Returns the path to the AOT store output directory.
#[inline]
pub(crate) fn aot_store_path() -> PathBuf {
    default_path().join("output")
}

/// Returns the path to the sled database directory.
#[inline]
pub(crate) fn sleddb_path() -> PathBuf {
    default_path().join("db")
}
