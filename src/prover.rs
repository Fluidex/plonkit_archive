// Most of this file is modified from source codes of [Matter Labs](https://github.com/matter-labs)
use bellman_ce::pairing::Engine;
use bellman_ce::{
    kate_commitment::{Crs, CrsForLagrangeForm, CrsForMonomialForm},
    plonk::{
        better_cs::cs::PlonkCsWidth4WithNextStepParams, commitments::transcript::keccak_transcript::RollingKeccakTranscript, prove,
        prove_by_steps, setup, transpile, Proof, SetupPolynomials, TranspilationVariant,
    },
    Circuit, ScalarEngine, SynthesisError,
};

pub const SETUP_MIN_POW2: u32 = 20;
pub const SETUP_MAX_POW2: u32 = 26;

pub struct SetupForProver<E: Engine> {
    setup_polynomials: SetupPolynomials<E, PlonkCsWidth4WithNextStepParams>,
    hints: Vec<(usize, TranspilationVariant)>,
    key_monomial_form: Crs<E, CrsForMonomialForm>,
    key_lagrange_form: Option<Crs<E, CrsForLagrangeForm>>,
}

impl<E: Engine> SetupForProver<E> {
    pub fn prepare_setup_for_prover<C: Circuit<E> + Clone>(
        circuit: C,
        key_monomial_form: Crs<E, CrsForMonomialForm>,
        key_lagrange_form: Option<Crs<E, CrsForLagrangeForm>>,
    ) -> Result<Self, anyhow::Error> {
        let hints = transpile(circuit.clone())?;
        let setup_polynomials = setup(circuit, &hints)?;
        let size = setup_polynomials.n.next_power_of_two().trailing_zeros();
        let setup_power_of_two = std::cmp::max(size, SETUP_MIN_POW2); // for exit circuit
        anyhow::ensure!(
            (SETUP_MIN_POW2..=SETUP_MAX_POW2).contains(&setup_power_of_two),
            "setup power of two is not in the correct range"
        );

        Ok(SetupForProver {
            setup_polynomials,
            hints,
            key_monomial_form,
            key_lagrange_form,
        })
    }

    pub fn prove<C: Circuit<E> + Clone>(&self, circuit: C) -> Result<Proof<E, PlonkCsWidth4WithNextStepParams>, SynthesisError> {
        match &self.key_lagrange_form {
            Some(key_lagrange_form) => prove::<_, _, RollingKeccakTranscript<<E as ScalarEngine>::Fr>>(
                circuit,
                &self.hints,
                &self.setup_polynomials,
                &self.key_monomial_form,
                &key_lagrange_form,
            ),
            None => prove_by_steps::<_, _, RollingKeccakTranscript<<E as ScalarEngine>::Fr>>(
                circuit,
                &self.hints,
                &self.setup_polynomials,
                None,
                &self.key_monomial_form,
            ),
        }
    }
}
