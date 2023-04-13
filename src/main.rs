//#[macro_use]
//extern crate diesel;
//#[macro_use]
//extern crate diesel_migrations;

use dotenv::dotenv;

mod db_infra_pool;
mod models;
mod schema;

// docker run -itd -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -v /Users/georg/Postgres/data_test:/var/lib/postgresql/data --name postgresqltest postgres

fn main() {
    println!("Hello, world!");
    dotenv().ok();
    db_infra_pool::init();
}

#[cfg(test)]
mod tests {
    use crate::db_infra_pool;
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
}
