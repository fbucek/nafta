# Nafta
Creates temporary SQLite database for testing

## Minimal example

```rust
// Database
extern crate diesel;

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_user() {
        // Creates empty SQLite database in temporary folder
        let test_db = nafta::sqlite::TestDb::new();
        let pool = std::sync::Arc::new(test_db.pool);
        // Use code to work with the pool
        // You can check that database is removed
        let path = test_db.db_path.to_owned();
        drop(test_db);
        assert!(!path.exists());
    }
}
```
## Example with migration

```rust
// Database
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");

#[cfg(test)]
mod tests {

    #[test]
    async fn test_get_user() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = test_db
            .conn()
            .expect("Not possible to get pooled connection");

        // Database migration
        embedded_migrations::run(&conn).expect("Migration not possible to run");

        // Example method to get all users
        let all_user = db::users::get_all_users(test_db.pool);
        assert!(all_user.is_ok());
    }
}
```
