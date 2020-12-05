#![allow(clippy::needless_range_loop)]
extern crate bellman_ce;
extern crate rand;

use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::str;

use bellman_ce::{
    pairing::{bn256::Bn256, ff::PrimeField, ff::ScalarEngine, Engine},
    Circuit, ConstraintSystem, Index, LinearCombination, SynthesisError, Variable,
};


#[derive(Serialize, Deserialize)]
struct CircuitJson {
    pub constraints: Vec<Vec<BTreeMap<String, String>>>,
    #[serde(rename = "nPubInputs")]
    pub num_inputs: usize,
    #[serde(rename = "nOutputs")]
    pub num_outputs: usize,
    #[serde(rename = "nVars")]
    pub num_variables: usize,
}

pub type Constraint<E> = (
    Vec<(usize, <E as ScalarEngine>::Fr)>,
    Vec<(usize, <E as ScalarEngine>::Fr)>,
    Vec<(usize, <E as ScalarEngine>::Fr)>,
);

#[derive(Clone)]
pub struct R1CS<E: Engine> {
    pub num_inputs: usize,
    pub num_aux: usize,
    pub num_variables: usize,
    pub constraints: Vec<Constraint<E>>,
}

#[derive(Clone)]
pub struct CircomCircuit<E: Engine> {
    pub r1cs: R1CS<E>,
    pub witness: Option<Vec<E::Fr>>,
    pub wire_mapping: Option<Vec<usize>>,
    pub aux_offset: usize,
    // debug symbols
}

/// Our demo circuit implements this `Circuit` trait which
/// is used during paramgen and proving in order to
/// synthesize the constraint system.
impl<'a, E: Engine> Circuit<E> for CircomCircuit<E> {
    //noinspection RsBorrowChecker
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let witness = &self.witness;
        let wire_mapping = &self.wire_mapping;
        for i in 1..self.r1cs.num_inputs {
            cs.alloc_input(
                || format!("variable {}", i),
                || {
                    Ok(match witness {
                        None => E::Fr::from_str("1").unwrap(),
                        Some(w) => match wire_mapping {
                            None => w[i],
                            Some(m) => w[m[i]],
                        },
                    })
                },
            )?;
        }
        for i in 0..self.r1cs.num_aux {
            cs.alloc(
                || format!("aux {}", i + self.aux_offset),
                || {
                    Ok(match witness {
                        None => E::Fr::from_str("1").unwrap(),
                        Some(w) => match wire_mapping {
                            None => w[i + self.r1cs.num_inputs],
                            Some(m) => w[m[i + self.r1cs.num_inputs]],
                        },
                    })
                },
            )?;
        }

        let make_index = |index| {
            if index < self.r1cs.num_inputs {
                Index::Input(index)
            } else {
                Index::Aux(index - self.r1cs.num_inputs + self.aux_offset)
            }
        };
        let make_lc = |lc_data: Vec<(usize, E::Fr)>| {
            lc_data
                .iter()
                .fold(LinearCombination::<E>::zero(), |lc: LinearCombination<E>, (index, coeff)| {
                    lc + (*coeff, Variable::new_unchecked(make_index(*index)))
                })
        };
        for (i, constraint) in self.r1cs.constraints.iter().enumerate() {
            cs.enforce(
                || format!("constraint {}", i),
                |_| make_lc(constraint.0.clone()),
                |_| make_lc(constraint.1.clone()),
                |_| make_lc(constraint.2.clone()),
            );
        }
        Ok(())
    }
}

pub fn witness_from_json_file<E: Engine>(filename: &str) -> Vec<E::Fr> {
    let reader = OpenOptions::new().read(true).open(filename).expect("unable to open.");
    witness_from_json::<E, BufReader<File>>(BufReader::new(reader))
}

pub fn witness_from_json<E: Engine, R: Read>(reader: R) -> Vec<E::Fr> {
    let witness: Vec<String> = serde_json::from_reader(reader).unwrap();
    witness.into_iter().map(|x| E::Fr::from_str(&x).unwrap()).collect::<Vec<E::Fr>>()
}

pub fn r1cs_from_json_file<E: Engine>(filename: &str) -> R1CS<E> {
    let reader = OpenOptions::new().read(true).open(filename).expect("unable to open.");
    r1cs_from_json(BufReader::new(reader))
}

pub fn r1cs_from_json<E: Engine, R: Read>(reader: R) -> R1CS<E> {
    let circuit_json: CircuitJson = serde_json::from_reader(reader).unwrap();

    let num_inputs = circuit_json.num_inputs + circuit_json.num_outputs + 1;
    let num_aux = circuit_json.num_variables - num_inputs;

    let convert_constraint = |lc: &BTreeMap<String, String>| {
        lc.iter()
            .map(|(index, coeff)| (index.parse().unwrap(), E::Fr::from_str(coeff).unwrap()))
            .collect_vec()
    };

    let constraints = circuit_json
        .constraints
        .iter()
        .map(|c| (convert_constraint(&c[0]), convert_constraint(&c[1]), convert_constraint(&c[2])))
        .collect_vec();

    R1CS {
        num_inputs,
        num_aux,
        num_variables: circuit_json.num_variables,
        constraints,
    }
}

pub fn r1cs_from_bin<R: Read>(reader: R) -> Result<(R1CS<Bn256>, Vec<usize>), std::io::Error> {
    let file = crate::r1cs_reader::read(reader)?;
    let num_inputs = (1 + file.header.n_pub_in + file.header.n_pub_out) as usize;
    let num_variables = file.header.n_wires as usize;
    let num_aux = num_variables - num_inputs;
    Ok((
        R1CS {
            num_aux,
            num_inputs,
            num_variables,
            constraints: file.constraints,
        },
        file.wire_mapping.iter().map(|e| *e as usize).collect_vec(),
    ))
}

pub fn r1cs_from_bin_file(filename: &str) -> Result<(R1CS<Bn256>, Vec<usize>), std::io::Error> {
    let reader = OpenOptions::new().read(true).open(filename).expect("unable to open.");
    r1cs_from_bin(BufReader::new(reader))
}
