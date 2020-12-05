use anyhow::format_err;
use bellman_ce::{
    kate_commitment::{Crs, CrsForLagrangeForm, CrsForMonomialForm},
    pairing::Engine,
    plonk::{better_cs::cs::PlonkCsWidth4WithNextStepParams, better_cs::keys::Proof, VerificationKey},
};
use std::fs::File;
use std::io::BufReader;

///
/// proof
///

pub fn load_proof<E: Engine>(filename: &str) -> Proof<E, PlonkCsWidth4WithNextStepParams> {
    Proof::<E, PlonkCsWidth4WithNextStepParams>::read(File::open(filename).expect("read proof file err")).expect("read proof err")
}

///
/// verification key
///

pub fn load_verification_key<E: Engine>(filename: &str) -> VerificationKey<E, PlonkCsWidth4WithNextStepParams> {
    VerificationKey::<E, PlonkCsWidth4WithNextStepParams>::read(File::open(filename).expect("read vk file err")).expect("read vk err")
}

///
/// universal setup
///

fn get_universal_setup_file_buff_reader(setup_file_name: &str) -> Result<BufReader<File>, anyhow::Error> {
    let setup_file =
        File::open(setup_file_name).map_err(|e| format_err!("Failed to open universal setup file {}, err: {}", setup_file_name, e))?;
    Ok(BufReader::with_capacity(1 << 29, setup_file))
}

pub fn load_key_monomial_form<E: Engine>(filename: &str) -> Crs<E, CrsForMonomialForm> {
    let mut buf_reader = get_universal_setup_file_buff_reader(filename).expect("read key_monomial_form file err");
    Crs::<E, CrsForMonomialForm>::read(&mut buf_reader).expect("read key_monomial_form err")
}

pub fn maybe_load_key_lagrange_form<E: Engine>(maybe_filename: Option<String>) -> Option<Crs<E, CrsForLagrangeForm>> {
    match maybe_filename {
        None => None,
        Some(filename) => {
            let mut buf_reader = get_universal_setup_file_buff_reader(&filename).expect("read key_lagrange_form file err");
            let key_lagrange_form = Crs::<E, CrsForLagrangeForm>::read(&mut buf_reader).expect("read key_lagrange_form err");
            Some(key_lagrange_form)
        }
    }
}

/// -----------------------

pub fn witness_from_json_file<E: Engine>(filename: &str) -> Vec<E::Fr> {
    let reader = OpenOptions::new().read(true).open(filename).expect("unable to open.");
    witness_from_json::<E, BufReader<File>>(BufReader::new(reader))
}

pub fn witness_from_json<E: Engine, R: Read>(reader: R) -> Vec<E::Fr> {
    let witness: Vec<String> = serde_json::from_reader(reader).unwrap();
    witness.into_iter().map(|x| E::Fr::from_str(&x).unwrap()).collect::<Vec<E::Fr>>()
}

pub fn load_r1cs_from_json_file<E: Engine>(filename: &str) -> R1CS<E> {
    let reader = OpenOptions::new().read(true).open(filename).expect("unable to open.");
    r1cs_from_json(BufReader::new(reader))
}

pub fn load_r1cs_from_json<E: Engine, R: Read>(reader: R) -> R1CS<E> {
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
