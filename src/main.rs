//#[macro_use]
//extern crate diesel;
//#[macro_use]
//extern crate diesel_migrations;

use crate::db_infra_pool::connection;
use crate::models::{NewPost, Post};
use axum::{routing, Router};
use axum_prometheus::{metrics_exporter_prometheus::PrometheusHandle, PrometheusMetricLayer};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tracing::*;

mod db_infra_pool;
mod listen;
mod models;
mod schema;

// docker run -itd -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -v /Users/georg/Postgres/data_test:/var/lib/postgresql/data --name postgresqltest postgres

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, db!");

    // Tracing!
    register_tracing();

    // Now trace the started message
    info!("Started Hello DB");

    // Metrics
    let (prometheus_layer, metric_handle) = define_metrics();

    // Prepare environment and initialize DB
    dotenv().ok();
    db_infra_pool::init();

    // Do a quick DB Test
    let connection = &mut connection();
    create_delete_post(connection);

    // Do Route stuff
    let router = create_router_with_prometheus(prometheus_layer, metric_handle);

    // Start Web Server at port 8080
    use tokio::signal::unix as usig;
    let mut shutdown = usig::signal(usig::SignalKind::terminate())?;
    let server = axum::Server::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(router.into_make_service())
        .with_graceful_shutdown(async move {
            shutdown.recv().await;
        });

    // Wait for either Watcher or WebServer to exit and write log hwo exited (the Watcher should never exit by its own)
    tokio::select! {
        _ = server => info!("axum server exited"),
    }

    // Finish
    Ok(())
}

fn register_tracing() {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_max_level(Level::INFO)
        .init();
}

fn define_metrics() -> (PrometheusMetricLayer<'static>, PrometheusHandle) {
    axum_prometheus::PrometheusMetricLayer::pair()
}

fn create_router_with_prometheus(prometheus_layer: PrometheusMetricLayer<'static>, metric_handle: PrometheusHandle) -> Router {
    let routers = create_router()
        .route("/actuator/prometheus", routing::get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);

    Router::new().nest("/hello", routers)
}

fn create_router() -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    Router::new()
        // Here the business routes later
        //.layer(Extension(reader_deployment))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        // Reminder: routes added *after* TraceLayer are not subject to its logging behavior
        .route("/actuator/health", routing::get(listen::health))
        .layer(cors)
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

    #[ctor::ctor]
    fn init() {
        println!("We are in init to initialize environment and database");
        dotenv().ok();
        db_infra_pool::init();
    }

    #[test]
    fn test_db_url() {
        //dotenv().ok();
        let pg_res = std::env::var("DATABASE_URL");

        println!("Postgres URL = {:?}", pg_res);

        let now: DateTime<Utc> = Utc::now();

        println!("UTC now is: {}", now);
        println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
        println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    }

    #[test]
    fn try_db_init() {
        //dotenv().ok();
        //db_infra_pool::init();
    }

    #[test]
    fn try_create_delete_post() {
        //dotenv().ok();
        //db_infra_pool::init();

        let connection = &mut connection();
        create_delete_post(connection);
    }
}
