use ndarray::Array2;
use serde_yaml::Value;

/// Dense matrix multiplication: C = A * B
/// Intended to be compute-bound for sufficiently large N.
pub fn run(config: &Value) {
    let n = config["matrix_size"]
        .as_u64()
        .expect("matrix_size must be set") as usize;

    // Deterministic-ish inputs (no RNG dependency):
    // Fill arrays with a simple function of indices.
    let a = Array2::from_shape_fn((n, n), |(i, j)| ((i + j) % 100) as f64 * 0.01);
    let b = Array2::from_shape_fn((n, n), |(i, j)| ((i * 3 + j * 7) % 100) as f64 * 0.01);

    // ndarray's dot does matrix multiplication.
    // This allocates the result C (that's fine for this benchmark).
    let c = a.dot(&b);

    // Prevent potential “dead code elimination” (unlikely, but good practice).
    std::hint::black_box(c);
}

