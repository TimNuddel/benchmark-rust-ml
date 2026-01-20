mod benchmarks;

use rayon::ThreadPoolBuilder;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::time::Instant;

#[derive(Deserialize)]
struct BaseConfig {
    runs: usize,
    warmup_runs: usize,
    num_threads: usize,
    output_dir: String,
}

fn main() {
    let config_path = env::args().nth(1).expect("Config path required");

    let base: BaseConfig =
        serde_yaml::from_reader(File::open("configs/base.yaml").unwrap()).unwrap();

    ThreadPoolBuilder::new()
        .num_threads(base.num_threads)
        .build_global()
        .unwrap();

    let start = Instant::now();
    benchmarks::dispatch(&config_path, &base);
    println!("Finished in {:?}", start.elapsed());
}
