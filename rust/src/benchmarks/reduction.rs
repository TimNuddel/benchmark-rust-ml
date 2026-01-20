use rayon::prelude::*;
use serde_yaml::Value;

/// Vector reduction (sum):
/// Intended to be memory-bandwidth-bound for large N.
pub fn run(config: &Value) {
    let n = config["vector_size"]
        .as_u64()
        .expect("vector_size must be set") as usize;

    // Deterministic-ish values without RNG:
    let x: Vec<f64> = (0..n).map(|i| (i % 100) as f64 * 0.01).collect();

    // Parallel reduction (sum). For a single-thread baseline later,
    // you can add an alternate implementation.
    let s: f64 = x.par_iter().copied().sum();

    std::hint::black_box(s);
}

