use ark_bn254::{Bn254, Fr};
use ark_groth16::{generate_random_parameters, create_random_proof, Proof, ProvingKey, VerifyingKey};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::rand::thread_rng;

pub struct DemoCircuit {
    pub a: Fr,
    pub b: Fr,
}

impl ConstraintSynthesizer<Fr> for DemoCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        use ark_r1cs_std::alloc::AllocVar;
        use ark_r1cs_std::fields::fp::FpVar;

        let a = FpVar::new_input(cs.clone(), || Ok(self.a))?;
        let b = FpVar::new_input(cs.clone(), || Ok(self.b))?;
        let c = FpVar::new_input(cs.clone(), || Ok(self.a * self.b))?;

        let ab = &a * &b;
        ab.enforce_equal(&c)?;
        Ok(())
    }
}

pub fn generate_keys_and_proof() -> (ProvingKey<Bn254>, VerifyingKey<Bn254>, Proof<Bn254>) {
    let rng = &mut thread_rng();
    let circuit = DemoCircuit {
        a: Fr::from(3u64),
        b: Fr::from(11u64),
    };

    let params = generate_random_parameters::<Bn254, _, _>(circuit, rng).unwrap();

    let proof = create_random_proof(DemoCircuit {
        a: Fr::from(3u64),
        b: Fr::from(11u64),
    }, &params, rng).unwrap();

    (params.pk, params.vk, proof)
}
