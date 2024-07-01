use std::net::TcpListener;

use sqlx::PgPool;
use zero_to_production::{configuration::get_configuration, startup::get_app_server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to DB");

    get_app_server(listener, db_pool)?.await
}
