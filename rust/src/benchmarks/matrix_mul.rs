use ndarray::Array2;

pub fn run(a: &Array2<f64>, b: &Array2<f64>) -> f64 {
    let c = a.dot(b);
    // return a scalar so work can't be optimized away
    c[(0, 0)]
}
