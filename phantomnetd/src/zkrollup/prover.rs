use blake3::Hasher;
use serde_json::Value;

pub fn compute_merkle_root(batch: &[Value]) -> String {
    let mut hashes: Vec<String> = batch.iter()
        .map(|node| blake3::hash(node.to_string().as_bytes()).to_hex().to_string())
        .collect();

    while hashes.len() > 1 {
        hashes = hashes.chunks(2).map(|pair| {
            let combined = format!("{}{}", pair[0], pair.get(1).unwrap_or(&pair[0]));
            blake3::hash(combined.as_bytes()).to_hex().to_string()
        }).collect();
    }

    hashes.first().cloned().unwrap_or_else(|| "0".repeat(64))
}

pub fn generate_dummy_rollup_proof(_merkle_root: &str) -> String {
    "rollup-proof-stub".to_string()
}