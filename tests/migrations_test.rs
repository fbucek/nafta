// Database
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
extern crate nafta;

use diesel::dsl::insert_into;
use diesel::prelude::*;
// Macro needed for: embedded_migrations::run(&conn)
embed_migrations!("tests/migrations");

// Define schema for Posts
table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
    }
}

mod tests {
    //#[cfg_attr(test, macro_use)]
    use super::*;

    #[derive(Queryable)]
    struct Post {
        id: i32,
        title: String,
        body: String,
    }

    #[derive(Insertable)]
    #[table_name = "posts"]
    struct NewPost<'a> {
        title: &'a str,
        body: &'a str,
    }

    #[test]
    fn test_init() {
        use super::posts::dsl::*;

        let test_db = nafta::sqlite::TestDb::new();
        // Path with database must exists
        let path = test_db.db_path.to_owned();
        assert!(path.exists());
        let conn = test_db
            .conn()
            .expect("Not possible to get pooled connection");

        embedded_migrations::run(&conn).expect("Migration not possible to run");

        let new_post = NewPost {
            title: "new post",
            body: "not empty body",
        };

        let inserted = insert_into(posts)
            .values(&new_post)
            .execute(&conn)
            .expect("Not possible to insert new post into database");
        assert_eq!(inserted, 1);

        // Get all values -> Must be only 1
        let last_post = posts
            .order(id.desc())
            .get_result::<Post>(&conn)
            .expect("Not possible to query Posts");

        assert_eq!(last_post.id, 1);
        assert_eq!(last_post.title, "new post");
        assert_eq!(last_post.body, "not empty body");

        // Path after TestDb is drop must not exists
        drop( conn); // First connection which holds ref to Pool must be droped!!!
        drop(test_db); // Drop TestDb
        assert!(!path.exists());
    }
}
