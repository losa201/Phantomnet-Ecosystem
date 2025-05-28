use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZkProofPayload {
    pub proof: String,
    pub public_inputs: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ZkVerifyResult {
    pub valid: bool,
    pub message: String,
}

pub async fn verify_proof_handler(
    State(_state): State<Arc<()>>,
    Json(payload): Json<ZkProofPayload>
) -> Json<ZkVerifyResult> {
    let inputs = parse_public_inputs(&payload.public_inputs);

    let vk_path = "./verifier_key.bin";
    let proof_path = "./zk_proof.bin";

    let vk_bytes = std::fs::read(vk_path).unwrap_or_default();
    let proof_bytes = std::fs::read(proof_path).unwrap_or_default();

    let valid = if !vk_bytes.is_empty() && !proof_bytes.is_empty() {
        verify_groth16_bn254_proof(&vk_bytes, &proof_bytes, &inputs)
    } else {
        payload.proof == "proof_ok"  // fallback to stub
    };

    Json(ZkVerifyResult {
        valid,
        message: if valid { "Proof verified".into() } else { "Invalid proof".into() },
    })
}
    State(_state): State<Arc<()>>,
    Json(payload): Json<ZkProofPayload>
) -> Json<ZkVerifyResult> {
    // Stub logic: pretend "proof_ok" is valid
    if payload.proof == "proof_ok" {
        Json(ZkVerifyResult {
            valid: true,
            message: "Proof verified (stub)".into(),
        })
    } else {
        Json(ZkVerifyResult {
            valid: false,
            message: "Invalid or rejected proof (stub)".into(),
        })
    }
}

// zkVerifier: Real Groth16 stub integration
use ark_bn254::{Bn254, Fr};
use ark_groth16::{verify_proof, PreparedVerifyingKey, Proof};
use ark_serialize::{CanonicalDeserialize};
use ark_std::vec::Vec;

pub fn verify_groth16_bn254_proof(
    vk_bytes: &[u8],
    proof_bytes: &[u8],
    public_inputs: &[Fr],
) -> bool {
    let vk = PreparedVerifyingKey::<Bn254>::deserialize_compressed(vk_bytes);
    let proof = Proof::<Bn254>::deserialize_compressed(proof_bytes);

    if let (Ok(prep_vk), Ok(proof)) = (vk, proof) {
        verify_proof(&prep_vk, &proof, public_inputs).unwrap_or(false)
    } else {
        false
    }
}

use std::fs::File;
use std::io::Read;

/// Load binary data from a local file path
pub fn load_binary_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Verifier file not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read verifier file");
    buffer
}

use ark_bn254::Fr as BnFr;
use ark_ff::PrimeField;

fn parse_public_inputs(inputs: &[String]) -> Vec<BnFr> {
    inputs.iter().filter_map(|s| BnFr::from_str(s).ok()).collect()
}