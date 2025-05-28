use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, prepare_verifying_key};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::rand::thread_rng;

#[macro_use]
extern crate ark_relations;

#[derive(Clone)]
struct SquareCircuit {
    pub x: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for SquareCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let x_val = self.x.ok_or(SynthesisError::AssignmentMissing)?;
        let x_var = cs.new_witness_variable(|| Ok(x_val))?;
        let x_squared_val = x_val * x_val;
        let x_squared_var = cs.new_input_variable(|| Ok(x_squared_val))?;

        cs.enforce_constraint(
            lc!() + x_var,
            lc!() + x_var,
            lc!() + x_squared_var,
        )?;

        Ok(())
    }
}

pub fn demo() {
    let rng = &mut thread_rng();
    let circuit = SquareCircuit { x: Some(Fr::from(3u64)) };

    let pk = Groth16::<Bn254>::generate_random_parameters_with_reduction(circuit.clone(), rng).unwrap();
    let proof = Groth16::<Bn254>::create_random_proof_with_reduction(circuit, &pk, rng).unwrap();

    let pvk = prepare_verifying_key(&pk.vk);
    let public_input = [Fr::from(9u64)]; // 3^2 = 9

    let verified = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_input).unwrap();
    println!("ZK Proof is valid: {}", verified);
}
