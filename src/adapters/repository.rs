

use redis::RedisResult;
use redis_cluster_async::{Connection,
                          redis::{cmd},
};
use redis_cluster_async::redis::aio::MultiplexedConnection;
use std::collections::{BTreeMap, BTreeSet};



// pub struct RepoClient {
//     pub db: ClusterClient,
// }
// impl RepoClient {
//     pub async fn new(settings: &RedisConfig) -> RepoClient {
//         let nodes = &settings.redis_uris;
//         RepoClient {
//             db: ClusterClient::open(nodes.clone()).unwrap(),
//         }
//     }
// }

pub struct RepoHash {
    pub value: BTreeMap<String, String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoHash {
    pub async fn set(data: RepoHash, mut conn: Connection<MultiplexedConnection>) -> RedisResult<()> {
        cmd("HMSET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut conn).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut conn).await?;
        }

        Ok(())
    }
    pub async fn get(key: String, mut conn: Connection<MultiplexedConnection>) -> RedisResult<RepoHash> {
        let info: BTreeMap<String, String> = cmd("HGETALL").arg(&key).query_async(&mut conn).await?;
        Ok(RepoHash {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoString {
    pub value: String,
    pub key: String,
    pub ttl: usize,
}

impl RepoString {
    pub async fn set(data: RepoString, mut conn: Connection<MultiplexedConnection>) -> RedisResult<()> {
        cmd("SET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut conn).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut conn).await?;
        }

        Ok(())
    }
    pub async fn get(
        key: String,
        mut conn: Connection<MultiplexedConnection>,
    ) -> RedisResult<RepoString> {
        let info: String = cmd("GET").arg(&key).query_async(&mut conn).await?;
        Ok(RepoString {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoList {
    pub value: Vec<String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoList {
    pub async fn set(data: RepoList, mut conn: Connection<MultiplexedConnection>) -> RedisResult<()> {
        cmd("RPUSH")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut conn).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut conn).await?;
        }

        Ok(())
    }
    pub async fn get(key: String, mut conn: Connection<MultiplexedConnection>) -> RedisResult<RepoList> {
        let info: Vec<String> = cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query_async(&mut conn).await?;
        Ok(RepoList {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoSet {
    pub value: BTreeSet<String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoSet {
    pub async fn set(data: RepoSet, mut conn: Connection<MultiplexedConnection>) -> RedisResult<()> {
        cmd("SADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut conn).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut conn).await?;
        }
        Ok(())
    }
    pub async fn get(key: String, mut conn: Connection<MultiplexedConnection>) -> RedisResult<RepoSet> {
        let info: BTreeSet<String> = cmd("SMEMBERS").arg(&key).query_async(&mut conn).await?;
        Ok(RepoSet {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoZSet {
    pub value: BTreeMap<String, f32>,
    pub key: String,
    pub ttl: usize,
}

impl RepoZSet {
    pub async fn set(data: RepoZSet, mut conn: Connection<MultiplexedConnection>) -> RedisResult<()> {
        cmd("ZADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut conn).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut conn).await?;
        }
        Ok(())
    }
    pub async fn get(key: String, mut conn: Connection<MultiplexedConnection>) -> RedisResult<RepoZSet> {
        let info: BTreeMap<String, f32> = cmd("ZRANGE")
            .arg(&key)
            .arg("-inf")
            .arg("+inf")
            .arg("WITHSCORES")
            .query_async(&mut conn).await?;

        Ok(RepoZSet {
            key,
            value: info,
            ttl: 0,
        })
    }
}
