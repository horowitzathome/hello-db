// See https://blog.logrocket.com/create-backend-api-with-rust-postgres/
//
use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use std::{env, error::Error};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection();
    let result = run_migrations(&mut conn);
    if let Err(error) = result {
        println!("Error in migration run {}", error);
    }
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn connection() -> DbConnection {
    POOL.get().expect("Failed to get db connection")
}
