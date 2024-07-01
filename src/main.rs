use std::net::TcpListener;

use zero_to_production::get_app_server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    get_app_server(listener)?.await
}
