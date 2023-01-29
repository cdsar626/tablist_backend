use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};

pub fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            //.route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            //.route("/subscriptions", web::post().to(subscribe))
            .app_data(db.clone())
        //.route("/{name}", web::get().to(greet))
    })
        .listen(listener)?
        .run();
    Ok(server)
}