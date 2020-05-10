# Nafta

[![Build Status](https://github.com/fbucek/nafta/workflows/build/badge.svg)](https://github.com/fbucek/nafta/actions)
[![Documentation](https://docs.rs/nafta/badge.svg)](https://docs.rs/nafta)
[![crates.io](https://meritbadge.herokuapp.com/nafta)](https://crates.io/crates/nafta)

Creates temporary SQLite database for testing using diesel.

```toml
[dev-dependencies]
nafta = "0.1"
```

## Minimal example

```rust
// Creates empty SQLite database in temporary folder
let test_db = nafta::sqlite::TestDb::new();

// Work with the connetion
let conn = test_db.conn();

// You can check that database file was really removed
let path = test_db.db_path.clone();
// Necessary to drop anything which can block file
drop(conn); 
// Dropping `test_db` to check it was really removed
drop(test_db);
assert!(!path.exists()); 
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

## Building on windows 

See build batch for windows: `.github/install-sqlite.bat`
