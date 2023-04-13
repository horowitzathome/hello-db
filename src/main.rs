//#[macro_use]
//extern crate diesel;
//#[macro_use]
//extern crate diesel_migrations;

use crate::db_infra_pool::connection;
use crate::models::{NewPost, Post};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};
use dotenv::dotenv;

mod db_infra_pool;
mod models;
mod schema;

// docker run -itd -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -v /Users/georg/Postgres/data_test:/var/lib/postgresql/data --name postgresqltest postgres

fn main() {
    println!("Hello, world!");

    dotenv().ok();
    db_infra_pool::init();

    let connection = &mut connection();

    create_delete_post(connection);
}

fn create_delete_post(connection: &mut PgConnection) {
    let now: DateTime<Utc> = Utc::now();
    let title = format!("A title at {}", now.to_rfc2822());
    let body = "This is the body of the post";

    let post = create_post(connection, &title, body);
    println!("Saved draft post {} with id {}", title, post.id);

    println!("Will now delete post again.");
    delete_post(connection, post.id);
}

fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table).values(&new_post).get_result(conn).expect("Error saving new post")
}

fn delete_post(conn: &mut PgConnection, id_arg: i32) {
    use crate::schema::posts::dsl::*;
    diesel::delete(posts.filter(id.eq(id_arg))).execute(conn).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{
        create_delete_post,
        db_infra_pool::{self, connection},
    };
    use chrono::{DateTime, Utc};
    use dotenv::dotenv;

    #[test]
    fn test_db_url() {
        dotenv().ok();
        let pg_res = std::env::var("DATABASE_URL");

        println!("Postgres URL = {:?}", pg_res);

        let now: DateTime<Utc> = Utc::now();

        println!("UTC now is: {}", now);
        println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
        println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    }

    #[test]
    fn try_db_init() {
        dotenv().ok();
        db_infra_pool::init();
    }

    #[test]
    fn try_create_delete_post() {
        dotenv().ok();
        db_infra_pool::init();

        let connection = &mut connection();
        create_delete_post(connection);
    }
}
