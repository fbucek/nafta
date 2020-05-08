use diesel::prelude::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Test database builder
/// 
pub struct TestDb {
    pub pool: Pool,
    pub db_path: std::path::PathBuf,
    pub tmp_dir: tempfile::TempDir,
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

// impl Drop for TestDb {
//     fn drop(&mut self) {
//         // Necesary to drop pool first 
//         let pool = self.pool.take();
//         drop(pool);
//         // self.pool = None;
//         // drop(&self.pool);
//         // Have to remove file before temp_dir goes out of scope
//         // @see https://docs.rs/tempfile/3.1.0/tempfile/struct.TempDir.html#resource-leaking
//         std::fs::remove_file(&self.db_path)
//             .expect(format!("Not possible to remove self.db_path: {:?}", self.db_path).as_str());
//     }
// }



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

        let mut dirpath: std::path::PathBuf = test_db.tmp_dir.path().to_path_buf();

        assert!(dirpath.exists());

        // Path after TestDb is drop must not exists
        let parent = dirpath.parent().unwrap();

        let list = std::fs::read_dir(&parent).unwrap();
        for item in list {
            println!("Name: {:?}", item);
        }

        drop(test_db);

        let list = std::fs::read_dir(&parent).unwrap();
        for item in list {
            println!("Name: {:?}", item);
        }

        assert!(!dirpath.exists());
        assert!(!path.exists());
    }
}
