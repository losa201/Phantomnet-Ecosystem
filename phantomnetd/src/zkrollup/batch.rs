use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn collect_valid_batch(batch_size: usize) -> Vec<Value> {
    let mut collected = vec![];
    if let Ok(file) = File::open("data/zktangle.log") {
        for line in BufReader::new(file).lines().flatten().rev() {
            if let Ok(v) = serde_json::from_str::<Value>(&line) {
                if v.get("proof") == Some(&Value::String("valid".into())) {
                    collected.push(v);
                    if collected.len() >= batch_size {
                        break;
                    }
                }
            }
        }
    }
    collected.reverse(); // Preserve original order
    collected
}