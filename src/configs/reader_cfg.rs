use config;
use config::Config;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RedisConfig {
    pub redis_strategy: String,
    pub redis_hostname: String,
    pub redis_port: u16,
    pub redis_uris: Vec<String>,
}

impl RedisConfig {
    pub fn new(field: HashMap<String, String>) -> RedisConfig {
        let mut config: RedisConfig = RedisConfig {
            redis_hostname: "".to_string(),
            redis_strategy: "".to_string(),
            redis_port: 0,
            redis_uris: vec![],
        };

        for (k, v) in field.iter() {
            match k.as_str() {
                "redis_strategy" => {
                    config.redis_strategy = v.to_string();
                }
                "redis_hostname" => {
                    config.redis_hostname = v.to_string();
                }
                "redis_port" => {
                    config.redis_port = v.parse().unwrap();
                }
                "redis_uris" => {
                    let mut uris: Vec<String> = vec![];
                    let splited: Vec<&str> = v.split(",").collect();
                    for uri in splited.iter() {
                        uris.push(uri.to_string());
                    }
                    config.redis_uris = uris;
                }
                _ => {}
            }
        }

        config
    }
}


pub struct LogConfig {
    pub log_level: String,
    pub log_output: String,
}

impl LogConfig {
    pub fn new(field: HashMap<String, String>) -> LogConfig {
        let mut config: LogConfig = LogConfig {
            log_level: "".to_string(),
            log_output: "".to_string(),
        };

        for (k, v) in field.iter() {
            match k.as_str() {
                "log_level" => {
                    config.log_level = v.to_string();
                }
                "output" => {
                    config.log_output = v.to_string();
                }
                _ => {}
            }
        }
        config
    }
}

pub struct SettingsReader {
    pub redis: RedisConfig,
    pub log: LogConfig,
}
impl SettingsReader {
    pub fn new(file_name: &str, app_prefix: &str) -> SettingsReader {
        let mut hsettings: Config = read_file(file_name);
        hsettings.merge(read_env(app_prefix)).unwrap();
        let redis: RedisConfig = RedisConfig::new(
            hsettings
                .clone()
                .try_into::<HashMap<String, String>>()
                .unwrap(),
        );
        let log: LogConfig = LogConfig::new(
            hsettings
                .clone()
                .try_into::<HashMap<String, String>>()
                .unwrap(),
        );
        SettingsReader { redis, log }
    }
}
fn read_file(file_name: &str) -> Config {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(file_name)).unwrap();
    settings
}
fn read_env(app_prefix: &str) -> Config {
    let mut settings = config::Config::default();
    settings
        .merge(config::Environment::with_prefix(app_prefix))
        .unwrap();
    settings
}
