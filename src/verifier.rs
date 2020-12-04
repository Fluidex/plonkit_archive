
/// Generates PLONK verification key for given circuit and saves key at the given path.
/// Returns used setup power of two. (e.g. 22)
fn generate_verification_key<E: Engine, C: Circuit<E> + Clone>(circuit: C) -> u32 {
    println!("Transpiling circuit");
    let (gates_count, transpilation_hints) = transpile_with_gates_count(circuit.clone()).expect("failed to transpile");
    let size_log2 = gates_count.next_power_of_two().trailing_zeros();
    assert!(size_log2 <= 20, "power of two too big {}, max: 20", size_log2);

    // exodus circuit is to small for the smallest setup
    let size_log2 = std::cmp::max(20, size_log2);
    println!("Reading setup file, gates_count: {}, pow2: {}", gates_count, size_log2);

    let key_monomial_form = get_universal_setup_monomial_form(size_log2).expect("Failed to read setup file.");

    println!("Generating setup");
    let setup = setup(circuit, &transpilation_hints).expect("failed to make setup");
    println!("Generating verification key");
    let verification_key = make_verification_key(&setup, &key_monomial_form).expect("failed to create verification key");
    verification_key
        .write(File::create(path).unwrap())
        .expect("Failed to write verification file."); // unwrap - checked at the function entry
    println!("Verification key successfully generated");
    size_log2
}