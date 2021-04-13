mod service;
mod configs;
mod adapters;
mod domain;
use crate::configs::reader_cfg::{SettingsReader, RedisConfig};
use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpServer, HttpResponse};
use regex::Regex;
use crate::domain::request::{Message};
use crate::service::hash_service::{set_hash, map_payload_to_repo_hash, get_hash, map_repo_hash};
use std::ptr::null;
use std::collections::{BTreeMap, BTreeSet};
use crate::service::string_service::{map_repo_string, get_string, map_payload_to_repo_string, set_string};
use crate::service::list_service::{get_list, map_payload_to_repo_list, set_list, map_repo_list};
use crate::service::set_service::{map_repo_set, get_set, map_payload_to_repo_set, set_set};
use crate::service::zset_service::{set_zset, map_payload_to_repo_zset, get_zset, map_repo_zset};

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
        "string"=>{
            set_string(&data,map_payload_to_repo_string(&info,key )).unwrap()
        }
        "list"=>{
            set_list(&data,map_payload_to_repo_list(&info,key )).unwrap()
        }
        "set"=>{
            set_set(&data,map_payload_to_repo_set(&info,key )).unwrap()
        }
        "zset"=>{
            set_zset(&data,map_payload_to_repo_zset(&info,key )).unwrap()
        }
        _ =>{}
    };
    HttpResponse::NoContent().body("")
}

async fn get_key(
    data: web::Data<&RedisConfig>,
    param: web::Query<Parameters>,
    path: web::Path<String>,
) -> HttpResponse{
    let mut m: Message= Message{ m_hash: None, m_string: None, m_list: None, m_set: None, m_zset: None, ttl: 0 };
    let key: String = get_key_from_path(path.to_string());
    match param.tip.as_str(){
        "hash"=>{
           let h: BTreeMap<String,String> = get_hash(&data, map_repo_hash(key));
            if m.m_hash.is_none() {
               return HttpResponse::NotFound().body("");
            }else{
                m.m_hash= Option::from(h);
            }
        }
        "string"=>{
            let s: String = get_string(&data, map_repo_string(key));
            if m.m_string.is_none() {
                return HttpResponse::NotFound().body("");
            }else{
                m.m_string= Option::from(s);
            }
        }
        "list"=>{
            let l: Vec<String> = get_list(&data, map_repo_list(key));
            if m.m_list.is_none() {
                return HttpResponse::NotFound().body("");
            }else{
                m.m_list= Option::from(l);
            }
        }
        "set"=>{
            let s: BTreeSet<String> = get_set(&data, map_repo_set(key));
            if m.m_set.is_none() {
                return HttpResponse::NotFound().body("");
            }else{
                m.m_set= Option::from(s);
            }
        }
        "zset"=>{
            let z: BTreeMap<String, f32> = get_zset(&data, map_repo_zset(key));
            if m.m_zset.is_none() {
                return HttpResponse::NotFound().body("");
            }else{
                m.m_zset= Option::from(z);
            }
        }
        _ =>{}
    };

    HttpResponse::Ok().body(serde_json::to_string_pretty(&m).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_config = &SETTINGS.redis;

    HttpServer::new(move || App::new()
        .data(redis_config)
        .service(web::resource("/api/keys/{path:.*}")
            .route(web::put().to(set_key))
            .route(web::get().to(get_key)))
    ).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn get_key_from_path(s: String)-> String{
    let re = Regex::new(r"/").unwrap();
    let result = re.replace_all(s.as_str(), ":");
    result.to_string()
}


#[cfg(test)]
mod test;