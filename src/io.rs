use bellman_ce::{
    pairing::{
        bn256::{Bn256, Fq, Fq2, G1Affine, G2Affine},
        ff::PrimeField,
        ff::ScalarEngine,
        CurveAffine, Engine,
    },
    plonk::{
        better_cs::cs::PlonkCsWidth4WithNextStepParams, better_cs::keys::Proof,
        commitments::transcript::keccak_transcript::RollingKeccakTranscript, VerificationKey,
    },
    source::QueryDensity,
    Circuit, ConstraintSystem, Index, LinearCombination, SynthesisError, Variable,
};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, Read};

pub fn load_proof_json_file<E: Engine>(filename: &str) -> Proof<E, PlonkCsWidth4WithNextStepParams> {
    let reader = OpenOptions::new().read(true).open(filename).expect("unable to open.");
    load_proof_json(BufReader::new(reader))
}

pub fn load_proof_json<R: Read, E: Engine>(reader: R) -> Proof<E, PlonkCsWidth4WithNextStepParams> {
    unimplemented!()
}

pub fn load_verification_key<E: Engine>(filename: &str) -> VerificationKey<E, PlonkCsWidth4WithNextStepParams> {
    VerificationKey::<E, PlonkCsWidth4WithNextStepParams>::read(File::open(filename).expect("read vk file err")).expect("read vk err")
}
