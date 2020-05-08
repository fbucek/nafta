//! # Nafta
//!
//! Creates temporary SQLite database for testing using diesel.
//! 
//!  # Minimal example
//! 
//! ```rust
//! // Creates empty SQLite database in temporary folder
//! let test_db = nafta::sqlite::TestDb::new();
//! // Work with the conn
//! let conn = test_db.conn();
//!
//! // You can check that database is removed
//! let path = test_db.db_path.clone();
//! // Necessary to drop anything which can block file
//! drop(conn); 
//! // Dropping `test_db` to check it was really removed
//! drop(test_db);
//! // sleep added due to problem on windows/github actions randomly failed #9
//! std::thread::sleep(std::time::Duration::from_millis(100));
//! assert!(!path.exists()); // Neccessary to test if path was removed
//! ```
//!
//! # Example with migration
//!
//! ```ignore
//! // Database
//! extern crate diesel;
//! 
//! #[macro_use]
//! extern crate diesel_migrations;
//! 
//! // This macro from `diesel_migrations` defines an `embedded_migrations` module
//! // containing a function named `run`. This allows the example to be run and
//! // tested without any outside setup of the database.
//! embed_migrations!("migrations");
//! 
//! #[cfg(test)]
//! mod tests {
//!
    //!     #[test]
//!     async fn test_get_user() {
//!         let test_db = nafta::sqlite::TestDb::new();
//!         let conn = test_db
//!             .conn()
//!             .expect("Not possible to get pooled connection");
//! 
//!         embedded_migrations::run(&conn).expect("Migration not possible to run");
//! 
//!         // Test
//!         let all_user = db::users::get_all_users(test_db.pool);
//!         assert!(all_user.is_ok());
//!     }
//! }
//! ```

pub mod sqlite;
