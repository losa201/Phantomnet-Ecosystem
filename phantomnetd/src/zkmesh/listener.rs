use std::net::{TcpListener};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use std::fs::{OpenOptions};
use serde_json::Value;
use std::collections::HashSet;
use crate::zkmesh::verify::verify_proof_groth16;

pub fn start_listener(bind: &str) {
    let listener = TcpListener::bind(bind).expect("Could not bind listener");
    println!("📡 zkMesh listener started at {}", bind);

    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let reader = BufReader::new(stream);
            let existing = read_existing_ids();
            let mut file = OpenOptions::new().create(true).append(true).open("data/zktangle.log").unwrap();

            for line in reader.lines().flatten() {
                if let Ok(val) = serde_json::from_str::<Value>(&line) {
                    if verify_proof_groth16(&val) {
                        if let Some(id) = val.get("id").and_then(|v| v.as_str()) {
                        if !existing.contains(id) {
                            let _ = writeln!(file, "{}", val.to_string());
                        }
                    }
                }
            }
        }
    });
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