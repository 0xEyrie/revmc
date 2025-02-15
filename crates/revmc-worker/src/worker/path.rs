use std::path::PathBuf;

/// Returns the default path
#[inline]
fn default_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_path = std::env::var("MACHINE_CODE_STORE").unwrap_or_else(|_| ".revmc".to_string());
    PathBuf::from(home_dir).join(config_path)
}

/// Returns the path to the store that save compiled result.
#[inline]
pub(crate) fn store_path() -> PathBuf {
    default_path().join("output")
}

/// Returns the path of the database.
#[inline]
pub(crate) fn db_path() -> PathBuf {
    default_path().join("db")
}
