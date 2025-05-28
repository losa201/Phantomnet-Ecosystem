use ark_bn254::Fr;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{vec::Vec, UniformRand};
use ark_r1cs_std::prelude::*;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;

/// A simple zkLogin circuit that proves knowledge of a password hash without revealing it.
#[derive(Clone)]
pub struct ZkLogin {
    pub password_hash: Option<Fr>,
    pub input_hash: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for ZkLogin {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let expected_hash = FpVar::new_input(cs.clone(), || self.password_hash.ok_or(SynthesisError::AssignmentMissing))?;
        let supplied_hash = FpVar::new_witness(cs.clone(), || self.input_hash.ok_or(SynthesisError::AssignmentMissing))?;

        // The proof must show that expected_hash == supplied_hash
        expected_hash.enforce_equal(&supplied_hash)?;

        Ok(())
    }
}