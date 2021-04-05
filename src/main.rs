mod configs;
use redis::cluster::ClusterClient;
use crate::configs::reader_cfg::SettingsReader;


fn main() {
   let settings:SettingsReader = SettingsReader::new("Settings.toml", "");
    println!("Teste: {}",settings.redis.redis_hostname);
    println!("teste2: {}",settings.redis.redis_uris[0].as_str());
}

