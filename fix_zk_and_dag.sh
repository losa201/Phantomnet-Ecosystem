#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🔧 Fixing phantomnet-zk..."
cat > phantomnet-zk/Cargo.toml <<TOML
[package]
name = "phantomnet-zk"
version = "0.1.0"
edition = "2021"

[dependencies]
ark-groth16 = "0.4"
ark-relations = "0.4"
ark-r1cs-std = "0.4"
ark-ff = "0.4"
ark-bn254 = "0.4"
rand = "0.8"
TOML

mkdir -p phantomnet-zk/src
cat > phantomnet-zk/src/lib.rs <<'RS'
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_groth16::generate_random_parameters;
use ark_groth16::create_random_proof;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::prelude::*;
use ark_bn254::{Bn254, Fr};
use ark_ff::UniformRand;
use rand::thread_rng;

pub struct ExampleCircuit;

impl ConstraintSynthesizer<Fr> for ExampleCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let _ = cs;
        Ok(())
    }
}

pub fn generate_proof() {
    let rng = &mut thread_rng();
    let _pk = generate_random_parameters::<Bn254, _, _>(ExampleCircuit, rng).unwrap();
}
RS

echo "🔧 Fixing rollup-dag..."
cat > rollup-dag/Cargo.toml <<TOML
[package]
name = "rollup-dag"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
uuid = "1.7"
petgraph = "0.8"
TOML

mkdir -p rollup-dag/src
cat > rollup-dag/src/lib.rs <<'RS'
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
}

pub fn create_graph() -> Graph<Node, ()> {
    let mut graph = Graph::<Node, ()>::new();
    let a = graph.add_node(Node { id: Uuid::new_v4(), name: "A".into() });
    let b = graph.add_node(Node { id: Uuid::new_v4(), name: "B".into() });
    graph.add_edge(a, b, ());
    graph
}
RS

echo "🧹 Cleaning..."
cargo clean

echo "🚀 Rebuilding..."
cargo build --workspace --all-targets --release
