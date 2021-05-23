mod adapters;
mod configs;
mod domain;


use mobc_redis_cluster::RedisClusterConnectionManager;

use crate::configs::reader_cfg::SettingsReader;
use crate::domain::request::Message;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use redis_cluster_async::{
    Client,
};
use crate::adapters::repository::{RepoHash, RepoString, RepoList, RepoSet, RepoZSet};
use std::borrow::Borrow;

type Pool = mobc::Pool<RedisClusterConnectionManager>;


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
    pool: web::Data<Pool>,
    param: web::Query<Parameters>,
    path: web::Path<String>,
    info: web::Json<Message>,
) -> HttpResponse {
    let key: String = path.to_string().replace("/", ":");


    match param.tip.as_str() {
        "hash" => {
            let value: BTreeMap<String, String> = info.m_hash.clone().unwrap();
            let req = RepoHash {
                value,
                key,
                ttl: info.ttl.clone(),
            };
            RepoHash::set(req, pool.borrow()).await.unwrap();
        },
        "string" => {
            let value: String = info.m_string.clone().unwrap();
            let req = RepoString {
                value,
                key,
                ttl: info.ttl.clone(),
            };
            RepoString::set(req, pool.borrow()).await.unwrap();
        },
        "list" =>{
            let value = info.m_list.clone().unwrap();
            let req = RepoList {
                value,
                key,
                ttl: info.ttl.clone(),
            };
            RepoList::set(req, pool.borrow()).await.unwrap();
        },
        "set" => {
            let value = info.m_set.clone().unwrap();
            let req = RepoSet {
                value,
                key,
                ttl: info.ttl.clone(),
            };
            RepoSet::set(req, pool.borrow()).await.unwrap();
        },
        "zset" => {
            let value = info.m_zset.clone().unwrap();
            let req = RepoZSet{
                value,
                key,
                ttl: info.ttl.clone(),
            };
            RepoZSet::set(req, pool.borrow()).await.unwrap();
        },
        _ => {()}
    };
    HttpResponse::NoContent().body("")
}

async fn get_key(
    pool: web::Data<Pool>,
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
    let key: String = path.to_string().replace("/", ":");

    match param.tip.as_str() {
        "hash" => {
            let h: BTreeMap<String, String> = RepoHash::get(key, pool.borrow()).await.unwrap().value;
            if h.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_hash = Option::from(h);
            }
        }
        "string" => {

            let s: String = RepoString::get(key, pool.borrow()).await.unwrap().value;
            if s.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_string = Option::from(s);
            }
        }
        "list" => {
            let l: Vec<String> = RepoList::get(key.clone(), pool.borrow()).await.unwrap().value;
            if l.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_list = Option::from(l);
            }
        }
        "set" => {
            let s: BTreeSet<String> = RepoSet::get(key.clone(), pool.borrow()).await.unwrap().value;
            if s.is_empty() {
                return HttpResponse::NotFound().body("");
            } else {
                m.m_set = Option::from(s);
            }
        }
        "zset" => {
            let z: BTreeMap<String, f32> = RepoZSet::get(key.clone(), pool.borrow()).await.unwrap().value;
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
    let manager = RedisClusterConnectionManager::new(client);
    let pool = Pool::builder().max_open(100).max_idle(50).build(manager);
    HttpServer::new(move || {
        App::new().data(pool.clone()).service(
            web::resource("/api/keys/{path:.*}")
                .route(web::put().to(set_key))
                .route(web::get().to(get_key)),
        )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}



#[cfg(test)]
mod test;
