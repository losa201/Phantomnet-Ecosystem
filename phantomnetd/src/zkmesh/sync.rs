use serde_json::Value;
use std::collections::HashSet;
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

pub fn fetch_and_merge_from_peer(peer_addr: &str) {
    if let Ok(mut stream) = TcpStream::connect_timeout(&peer_addr.parse().unwrap(), Duration::from_secs(2)) {
        let _ = stream.write_all(b"GET /api/tangle\n");

        let mut response = String::new();
        if let Ok(_) = stream.read_to_string(&mut response) {
            if let Ok(Value::Array(nodes)) = serde_json::from_str::<Value>(&response) {
                let existing = read_existing_ids();
                let mut file = OpenOptions::new().create(true).append(true).open("data/zktangle.log").unwrap();
                for node in nodes {
                    if let Some(id) = node.get("id").and_then(|x| x.as_str()) {
                        if !existing.contains(id) {
                            let _ = writeln!(file, "{}", node.to_string());
                        }
                    }
                }
            }
        }
    }
}

fn read_existing_ids() -> HashSet<String> {
    let mut ids = HashSet::new();
    if let Ok(f) = std::fs::File::open("data/zktangle.log") {
        for line in BufReader::new(f).lines().flatten() {
            if let Ok(Value::Object(obj)) = serde_json::from_str(&line) {
                if let Some(Value::String(id)) = obj.get("id") {
                    ids.insert(id.clone());
                }
            }
        }
    }
    ids
}