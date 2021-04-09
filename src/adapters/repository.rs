use redis::{Commands, RedisResult, RedisError};
use crate::configs::reader_cfg::RedisConfig;
use redis::cluster::{ ClusterClient};
use std::collections::{BTreeMap, HashMap};

pub struct RepoClient{
    pub db: ClusterClient,

}
impl RepoClient{
    pub fn new(settings: &RedisConfig) -> RepoClient{
        let nodes = &settings.redis_uris;
        let client = ClusterClient::open(nodes.clone()).unwrap();
        let repo: RepoClient = RepoClient{
            db: client,
        };
        repo
    }
}

pub struct RepoHash{
    pub value: BTreeMap<String, String>,
    pub key: String,
    pub ttl: u32,
}

impl RepoHash {
    pub fn set(data: RepoHash, repo_client: RepoClient) -> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let _: () = redis::cmd("HSET")
            .arg(data.key).arg(data.ttl)
            .arg(data.value)
            .query(&mut conn)
            .expect("failed to execute HSET");
        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoHash>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let info: BTreeMap<String, String> = redis::cmd("HGETALL")
            .arg(&key)
            .query(&mut conn)
            .expect("failed to execute HGETALL");
        let result: RepoHash= RepoHash{
            key: key,
            value: info,
            ttl: 0
        };
        Ok(result)
    }

}


fn hash(client: ClusterClient) {

    let mut conn = client.get_connection().unwrap();

    println!("******* Running HASH commands *******");

    let mut driver: BTreeMap<String, String> = BTreeMap::new();
    let prefix = "redis-driver";

    driver.insert(String::from("name"), String::from("redis-rs"));
    driver.insert(String::from("version"), String::from("0.19.0"));
    driver.insert(
        String::from("repo"),
        String::from("https://github.com/mitsuhiko/redis-rs"),
    );

    let _: () = redis::cmd("HSET")
        .arg(format!("{}:{}", prefix, "rust"))
        .arg(driver)
        .query(&mut conn)
        .expect("failed to execute HSET");

        let info: BTreeMap<String, String> = redis::cmd("HGETALL")
            .arg(format!("{}:{}", prefix, "rust"))
            .query(&mut conn)
            .expect("failed to execute HGETALL");

    println!("info for rust redis driver: {:?}", info);

    let _: () = conn
        .hset_multiple(
            format!("{}:{}", prefix, "go"),
            &[
                ("name", "go-redis"),
                ("version", "8.4.6"),
                ("repo", "https://github.com/go-redis/redis"),
            ],
        )
        .expect("failed to execute HSET");

    let repo_name: String = conn
        .hget(format!("{}:{}", prefix, "go"), "repo")
        .expect("failed to execute HGET");

    println!("go redis driver repo name: {:?}", repo_name);
}

fn list(mut conn: redis::cluster::ClusterConnection) {
    println!("******* Running LIST commands *******");

    let list_name = "items";

    let _: () = redis::cmd("LPUSH")
        .arg(list_name)
        .arg("item-1")
        .query(&mut conn)
        .expect("failed to execute LPUSH for 'items'");

    let item: String = conn
        .lpop(list_name)
        .expect("failed to execute LPOP for 'items'");
    println!("first item: {}", item);

    let _: () = conn.rpush(list_name, "item-2").expect("RPUSH failed");
    let _: () = conn.rpush(list_name, "item-3").expect("RPUSH failed");

    let len: isize = conn
        .llen(list_name)
        .expect("failed to execute LLEN for 'items'");
    println!("no. of items in list = {}", len);

    let items: Vec<String> = conn
        .lrange(list_name, 0, len - 1)
        .expect("failed to execute LRANGE for 'items'");
    println!("listing items in list");

    for item in items {
        println!("item: {}", item)
    }
}

fn set(mut conn: redis::cluster::ClusterConnection) {
    println!("******* Running SET commands *******");

    let set_name = "users";

    let _: () = conn
        .sadd(set_name, "user1")
        .expect("failed to execute SADD for 'users'");
    let _: () = conn
        .sadd(set_name, "user2")
        .expect("failed to execute SADD for 'users'");

    let ismember: bool = redis::cmd("SISMEMBER")
        .arg(set_name)
        .arg("user1")
        .query(&mut conn)
        .expect("failed to execute SISMEMBER for 'users'");
    println!("does user1 exist in the set? {}", ismember); //true

    let users: Vec<String> = conn.smembers(set_name).expect("failed to execute SMEMBERS");
    println!("listing users in set"); //true

    for user in users {
        println!("user: {}", user)
    }
}


