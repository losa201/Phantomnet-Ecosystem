use serde::{Serialize, Deserialize};
use serde_json;
use blake3;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: u64,
    pub payload: String,
}

pub fn hash_message(msg: &Message) -> String {
    let serialized = serde_json::to_string(msg).unwrap();
    blake3::hash(serialized.as_bytes()).to_hex().to_string()
}
