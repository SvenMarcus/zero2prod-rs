use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration, startup::run};

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let config = configuration::get_configuration().expect("Failed to read configuration file.");
    let connection_str = config.database.connection_string();
    let pool = PgPool::connect(&connection_str)
        .await
        .expect("Could not connect to DB");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Could not bind port");
    run(listener, pool)?.await
}
