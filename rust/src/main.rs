mod benchmarks;

use rayon::ThreadPoolBuilder;
use serde::Deserialize;
use std::env;
use std::fs::File;

#[derive(Deserialize, Clone)]
pub struct BaseConfig {
    pub num_inputs: usize,
    pub runs_per_input: usize,
    pub warmup_runs: usize,
    pub seed: u64,
    pub input_dir: String,
    pub num_threads: usize,
    pub output_dir: String,
}

fn main() {
    let config_path = env::args().nth(1).expect("Config path required");

    let base: BaseConfig =
        serde_yaml::from_reader(File::open("configs/base.yaml").unwrap()).unwrap();

    ThreadPoolBuilder::new()
        .num_threads(base.num_threads)
        .build_global()
        .unwrap();

    benchmarks::dispatch(&config_path, &base);
}
