mod service;
mod configs;
mod adapters;
use crate::configs::reader_cfg::SettingsReader;
use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::adapters::repository::RepoClient;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: SettingsReader =
        SettingsReader::new("Settings.toml", "");
}
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


async fn set(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_config = &SETTINGS.redis;


    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

