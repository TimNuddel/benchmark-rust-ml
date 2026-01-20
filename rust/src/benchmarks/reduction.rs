use rayon::prelude::*;

pub fn run(x: &[f64]) -> f64 {
    x.par_iter().copied().sum()
}
