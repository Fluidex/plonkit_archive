#!/bin/bash
set -ex
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
TOOL_DIR=$DIR"/contrib"
CIRCUIT_DIR=$DIR"/testdata/poseidon"
SETUP_DIR=$DIR"/keys/setup"

# from zksync/infrastructure/zk/src/run/run.ts
echo "Step1: download universal setup file"
pushd keys/setup
axel -c https://universal-setup.ams3.digitaloceanspaces.com/setup_2^20.key || true
popd

echo "Step2: compile circuit and calculate witness using snarkjs"
. $TOOL_DIR/process_circom_circuit.sh

echo "Step3: test prove and verify"
RUST_LOG=info cargo test --release simple_plonk_test

echo "Step4: prove with key_monomial_form"
cargo run --release prove -o 20 -m $CIRCUIT_DIR/setup_2^20.key -s plonk -c $CIRCUIT_DIR/circuit.r1cs.json -w $CIRCUIT_DIR/witness.json

# TODO:
# echo "Step5: dump l....."
# cargo run --release prove -s plonk -c $CIRCUIT_DIR/circuit.r1cs.json -w $CIRCUIT_DIR/witness.json

# # TODO:
# echo "Step6: prove with key_monomial_form & key_lagrange_form"
# cargo run --release prove -s plonk -c $CIRCUIT_DIR/circuit.r1cs.json -w $CIRCUIT_DIR/witness.json

echo "Step7: verify"
cargo run --release verify -s plonk -p $CIRCUIT_DIR/proof.bin -v $CIRCUIT_DIR/vk.bin
