use ark_bn254::Fr;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::prelude::*;
use ark_r1cs_std::fields::fp::FpVar;

/// zkVote proves a commitment equals choice + salt (simple demonstration)
#[derive(Clone)]
pub struct ZkVote {
    pub committed: Option<Fr>,
    pub choice: Option<Fr>,
    pub salt: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for ZkVote {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let committed_var = FpVar::new_input(cs.clone(), || self.committed.ok_or(SynthesisError::AssignmentMissing))?;
        let choice_var = FpVar::new_witness(cs.clone(), || self.choice.ok_or(SynthesisError::AssignmentMissing))?;
        let salt_var = FpVar::new_witness(cs.clone(), || self.salt.ok_or(SynthesisError::AssignmentMissing))?;
        let hash = choice_var + salt_var;
        hash.enforce_equal(&committed_var)?;
        Ok(())
    }
}