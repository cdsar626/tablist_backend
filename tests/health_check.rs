use std::net::TcpListener;
use mongodb::{options::ClientOptions, Client, Database};
//bson::doc
use tablist_server::configuration::{get_configuration, DatabaseSettings};
use tablist_server::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: Database,
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> TestApp {
    // Bind random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    // Read config
    let mut configuration = get_configuration().expect("Failed to read configuration");
    // Change database to test database
    configuration.database.database_name = format!("Tests_Rust");
    // DB Pool connection
    let db = configure_database(&configuration.database).await;

    let server = run(listener, db.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: db,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> Database {
    // Connect to Postgres
    let client_options = ClientOptions::parse(&config.connection_string(),)
        .await.expect("Failed to parse to Mongo Atlas");
    let client = Client::with_options(client_options).expect("Failed to connect to Mongo Atlas");
    let db = client.database(&config.database_name);
    // Create new database
    /* Not this time
    connection
        .execute(
            format!(
                r#"
    CREATE DATABASE "{}";
    "#,
                config.database_name
            )
                .as_str(),
        )
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to pool connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

        */
    db
}