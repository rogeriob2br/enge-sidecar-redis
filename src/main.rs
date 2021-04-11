mod service;
mod configs;
mod adapters;
mod domain;
use crate::configs::reader_cfg::{SettingsReader, RedisConfig};


use serde::{Serialize, Deserialize};

use actix_web::{web, App, HttpServer, HttpResponse};



use regex::Regex;

use crate::domain::request::{Message};
use crate::service::hash_service::{set_hash, map_payload_to_repo_hash};


#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: SettingsReader =
        SettingsReader::new("Settings.toml", "");
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameters{
    #[serde(rename = "type")]
    tip: String
}
async fn set_key(
    data: web::Data<&RedisConfig>,
    param: web::Query<Parameters>,
    path: web::Path<String>,
    info: web::Json<Message>
) -> HttpResponse{
    let key: String = get_key_from_path(path.to_string());


    match param.tip.as_str(){
        "hash"=>{
            set_hash(&data,map_payload_to_repo_hash(&info,key )).unwrap()
        }
        _ =>{}
    };
    HttpResponse::NoContent().body("")

}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_config = &SETTINGS.redis;

    HttpServer::new(move || App::new()
        .data(redis_config)
        .service(
            web::resource("/api/keys/{path:.*}").route(web::put().to(set_key))
        ) ).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn get_key_from_path(s: String)-> String{
    let re = Regex::new(r"/").unwrap();
    let result = re.replace_all(s.as_str(), ":");
    result.to_string()
}
