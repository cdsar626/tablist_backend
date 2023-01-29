use std::net::TcpListener;
use tablist_server::startup::run;
use tablist_server::configuration::get_configuration;
use mongodb::{bson::doc, options::ClientOptions, Client};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read config");
    let client_options = ClientOptions::parse(&configuration.database.connection_string(),)
        .await.expect("Failed to parse to Mongo Atlas");
    let client = Client::with_options(client_options).expect("Failed to connect to Mongo Atlas");
    let db = client.database(&configuration.database.database_name);

    let server_address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(server_address)?;
    run(listener, db)?.await
}