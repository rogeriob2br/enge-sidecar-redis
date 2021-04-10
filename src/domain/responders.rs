use serde::{Serialize};
use serde_json::Result;


#[derive(Deserialize)]
pub struct hashmap {
    hash: BTreeMap<String, String>
}
