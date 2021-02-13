use crate::controllers::{
    health_check::health_check,
    training_masters::{create_training_master, find_training_master},
};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use mongodb::Database;
use std::net::TcpListener;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, database: Database) -> Result<Server, std::io::Error> {
    let database = Data::new(Arc::new(database));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .data(web::JsonConfig::default().limit(4096))
            .route("/health_check", web::get().to(health_check))
            .route("/training_master", web::post().to(create_training_master))
            .route("/training_master", web::get().to(find_training_master))
            .app_data(database.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}