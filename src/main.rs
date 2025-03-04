use dotenv::dotenv;
use env_logger::Env;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

use realworld::configuration::get_config;
use realworld::startup::run;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_config().expect("failed to get config");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", config.application_port);
    println!("Server address: {}", address);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?
        .await
        .expect("Failed to bind");
    Ok(())
}
