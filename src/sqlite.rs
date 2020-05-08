use diesel::prelude::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Test database builder
/// 
pub struct TestDb {
    pub tmp_dir: tempfile::TempDir,
    pub db_path: std::path::PathBuf,
    pub pool: Pool,
}

impl TestDb {
    /// Creates empty SQLite database using `tempfile` ( file: `test.db` folder based on `CARGO_PKG_NAME` )
    pub fn new() -> TestDb {
        // Create temporary dir where db will be stored
        let tmp_dir = tempfile::Builder::new()
            .prefix(env!("CARGO_PKG_NAME"))
            .rand_bytes(5)
            .tempdir()
            .expect("not possible to create tempfile");

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

    /// Pooled connection
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

        let dirpath = test_db.tmp_dir.path().to_path_buf();
        assert!(dirpath.exists());

        // Path after TestDb is drop must not exists
        
        if dirpath.exists() {
            let list = std::fs::read_dir(&dirpath).unwrap();
            for item in list {
                println!("Name: {:?}", item);
            }
        }
        
        drop(test_db);


        let list = std::fs::read_dir(&dirpath).unwrap();
        for item in list {
            println!("Name: {:?}", item);
        }


        assert!(!dirpath.exists());
        assert!(!path.exists());
    }
}
