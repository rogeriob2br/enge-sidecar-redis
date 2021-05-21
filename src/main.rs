mod adapters;
mod configs;
mod domain;
mod service;

use crate::configs::reader_cfg::SettingsReader;
use crate::domain::request::Message;
use crate::service::hash_service::{get_hash, map_payload_to_repo_hash, map_repo_hash, set_hash};
use crate::service::list_service::{get_list, map_payload_to_repo_list, map_repo_list, set_list};
use crate::service::set_service::{get_set, map_payload_to_repo_set, map_repo_set, set_set};
use crate::service::string_service::{
    get_string, map_payload_to_repo_string, map_repo_string, set_string,
};
use crate::service::zset_service::{get_zset, map_payload_to_repo_zset, map_repo_zset, set_zset};
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use redis_cluster_async::{
    redis::{ Commands},
    Client,
};
use crate::adapters::repository::RepoHash;


#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: SettingsReader = SettingsReader::new("app");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    tip: String,
}

async fn set_key(
    data: web::Data<Client>,
    param: web::Query<Parameters>,
    path: web::Path<String>,
    info: web::Json<Message>,
) -> HttpResponse {
    let key: String = path.to_string().replace("/", ":");
    let client = data.get_ref();
    let conn = client.get_connection().await.unwrap();
    match param.tip.as_str() {
        "hash" => {
            let value: BTreeMap<String, String> = info.m_hash.clone().unwrap();
            let req = RepoHash {
                value,
                key,
                ttl: info.ttl.clone(),
            };
            RepoHash::set(req, conn).await.unwrap();
        },
        "string" => set_string(conn, map_payload_to_repo_string(&info, key)).await.unwrap(),
        "list" => set_list(conn, map_payload_to_repo_list(&info, key)).await.unwrap(),
        "set" => set_set(conn, map_payload_to_repo_set(&info, key)).await.unwrap(),
        "zset" => set_zset(conn, map_payload_to_repo_zset(&info, key)).await.unwrap(),
        _ => {()}
    };
    HttpResponse::NoContent().body("")
}

async fn get_key(
    data: web::Data<Client>,
    param: web::Query<Parameters>,
    path: web::Path<String>,
) -> HttpResponse {
    let mut m: Message = Message {
        m_hash: None,
        m_string: None,
        m_list: None,
        m_set: None,
        m_zset: None,
        ttl: 0,
    };
    let key: String = path.to_string().replace("/", ":");;
    let client = data.get_ref();
    let conn = client.get_connection().await.unwrap();

    match param.tip.as_str() {
        "hash" => {
            let req = RepoHash {
                value: Default::default(),
                key,
                ttl: 0,
            };
            let h: BTreeMap<String, String> = RepoHash::get(req.key, conn).await.unwrap().value;
            if h.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_hash = Option::from(h);
            }
        }
        "string" => {
            let s: String = get_string(conn, map_repo_string(key.clone())).await;
            if s.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_string = Option::from(s);
            }
        }
        "list" => {
            let l: Vec<String> = get_list(conn, map_repo_list(key.clone())).await;
            if l.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_list = Option::from(l);
            }
        }
        "set" => {
            let s: BTreeSet<String> = get_set(conn, map_repo_set(key.clone())).await;
            if s.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_set = Option::from(s);
            }
        }
        "zset" => {
            let z: BTreeMap<String, f32> = get_zset(conn, map_repo_zset(key.clone())).await;
            if z.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_zset = Option::from(z);
            }
        }
        _ => {}
    };

    HttpResponse::Ok().body(serde_json::to_string_pretty(&m).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_config = &SETTINGS.redis;
    let client = Client::open(redis_config.redis_uris.clone()).unwrap();

    HttpServer::new(move || {
        App::new().data(client.clone()).service(
            web::resource("/api/keys/{path:.*}")
                .route(web::put().to(set_key))
                .route(web::get().to(get_key)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}



#[cfg(test)]
mod test;
