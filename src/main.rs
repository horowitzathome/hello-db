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
    use dotenv::dotenv;

    #[test]
    fn test_db_url() {
        dotenv().ok();
        let pg_res = std::env::var("DATABASE_URL");

        println!("Postgres URL = {:?}", pg_res);
    }

    #[test]
    fn try_db_init() {
        dotenv().ok();
        db_infra_pool::init();
    }
}
