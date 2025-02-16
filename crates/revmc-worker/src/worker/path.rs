use std::path::PathBuf;

/// Returns the default path
fn default_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_path = {
        #[cfg(test)]
        {
            ".test".to_string()
        }
        #[cfg(not(test))]
        {
            std::env::var("WORKER_STORE").unwrap_or_else(|_| ".revmc".to_string())
        }
    };
    PathBuf::from(home_dir).join(config_path)
}

/// Returns the path to the store that save compiled result.
pub(crate) fn store_path() -> PathBuf {
    default_path().join("output")
}

/// Returns the path of the database.
pub(crate) fn db_path() -> PathBuf {
    default_path().join("db")
}
