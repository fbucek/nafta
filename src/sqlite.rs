use diesel::prelude::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// When crated database it is necessary to up
pub struct TestDb {
    pub tmp_dir: tempdir::TempDir,
    pub db_path: std::path::PathBuf,
    pub pool: Pool,
}

impl TestDb {
    /// Helper method to create database for testing
    pub fn new() -> TestDb {
        // Create temporary dir where db will be stored
        let tmp_dir =
            tempdir::TempDir::new(env!("CARGO_PKG_NAME")).expect("Not possible to create tempdir");

        let db_path = tmp_dir.path().join("test.db");

        // Connection manager
        let manager = r2d2::ConnectionManager::<SqliteConnection>::new(db_path.to_str().unwrap());
        // Creates database if does not exists
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        TestDb {
            tmp_dir,
            db_path,
            pool,
        }
    }

    /// Get connections
    pub fn conn(&self) -> Option<PooledConnection<ConnectionManager<SqliteConnection>>> {
        // Result -> Ok
        self.pool.get().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expecting that created database will be deleted upon TestDb drop ( out of scope )
    #[test]
    fn test_lifecycle() {
        let test_db = TestDb::new();

        // Path with database must exists
        let path = test_db.db_path.to_owned();
        assert!(path.exists());

        // Path after TestDb is drop must not exists
        drop(test_db);
        assert!(!path.exists());
    }
}
