use std::net::TcpListener;

use sqlx::PgPool;
use zero_to_production::{
    configuration::get_configuration,
    startup::get_app_server,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_subscriber(get_subscriber(
        "zero2prod".to_string(),
        "info".to_string(),
        std::io::stdout,
    ));

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let db_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to establish lazy DB pool");

    get_app_server(listener, db_pool)?.await
}
