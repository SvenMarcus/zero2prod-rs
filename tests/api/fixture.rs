use std::net::TcpListener;

use reqwest::{RequestBuilder, Response};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{self, DatabaseSettings};
use zero2prod::startup;

const ADDRESS: &str = "127.0.0.1";

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        let bind_address = format!("{}:0", ADDRESS);
        let listener = create_listener(bind_address);
        let port = listener.local_addr().unwrap().port();

        let pool = db_pool().await;
        let server = startup::run(listener, pool.clone()).expect("Failed to bind address");
        tokio::spawn(server);
        TestApp {
            address: format!("http://{}:{}", ADDRESS, port),
            db_pool: pool,
        }
    }
}

pub async fn send(request: RequestBuilder) -> Response {
    request.send().await.expect("Failed to execute request")
}

async fn db_pool() -> PgPool {
    let config = configuration::get_configuration().expect("Could not read configuration.");
    configure_db(&config.database).await
}

async fn configure_db(database_settings: &DatabaseSettings) -> sqlx::Pool<sqlx::Postgres> {
    let unique_db = DatabaseSettings {
        database_name: Uuid::new_v4().to_string(),
        ..database_settings.clone()
    };
    let connection_str = unique_db.connection_string_without_db();

    let mut connection = PgConnection::connect(&connection_str)
        .await
        .expect("Failed to connect to DB.");

    connection
        .execute(format!(r#"create database "{}";"#, unique_db.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect(&unique_db.connection_string())
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database.");
    pool
}

fn create_listener(bind_address: String) -> TcpListener {
    TcpListener::bind(bind_address).expect("Failed to bind port")
}
