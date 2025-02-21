use std::fs;

use crate::{db_path, sc_db_path, store_path};

pub(super) struct TestEnvGuard;

impl TestEnvGuard {
    pub(super) fn new() -> Self {
        let store = store_path();
        let db = db_path();
        let sc_db = sc_db_path();

        if store.exists() {
            let _ = fs::remove_dir_all(&store);
        }

        if db.exists() {
            let _ = fs::remove_dir_all(&db);
        }

        if sc_db.exists() {
            let _ = fs::remove_dir_all(&sc_db);
        }
        Self
    }
}

impl Drop for TestEnvGuard {
    fn drop(&mut self) {
        let store = store_path();
        let db = db_path();
        let sc_db = sc_db_path();

        if store.exists() {
            let _ = fs::remove_dir_all(&store);
        }

        if db.exists() {
            let _ = fs::remove_dir_all(&db);
        }

        if sc_db.exists() {
            let _ = fs::remove_dir_all(&sc_db);
        }
    }
}
