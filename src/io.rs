use bellman_ce::{
    pairing::{
        bn256::{Bn256, Fq, Fq2, G1Affine, G2Affine},
        ff::PrimeField,
        ff::ScalarEngine,
        CurveAffine, Engine,
    },
    plonk::{better_cs::cs::PlonkCsWidth4WithNextStepParams, Proof as PlonkProof},
    source::QueryDensity,
    Circuit, ConstraintSystem, Index, LinearCombination, SynthesisError, Variable,
};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, Read};

pub fn load_proof_json_file<E: Engine>(filename: &str) -> Proof<Bn256> {
    let reader = OpenOptions::new().read(true).open(filename).expect("unable to open.");
    load_proof_json(BufReader::new(reader))
}

pub fn load_proof_json<R: Read>(reader: R) -> Proof<Bn256> {
    unimplement!()
}
