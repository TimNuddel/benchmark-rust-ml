pub mod matrix_mul;
pub mod reduction;

use crate::BaseConfig;
use csv::Writer;
use ndarray::{Array1, Array2};
use ndarray_npy::read_npy;
use serde_yaml::Value;
use std::fs::{create_dir_all, File};
use std::time::Instant;

fn load_yaml_value(path: &str) -> Value {
    serde_yaml::from_reader(File::open(path).unwrap()).unwrap()
}

pub fn dispatch(config_path: &str, base: &BaseConfig) {
    let cfg: Value = load_yaml_value(config_path);
    let benchmark = cfg["benchmark"].as_str().unwrap();

    create_dir_all(&base.output_dir).unwrap();

    let total_timed_runs = base.num_inputs * base.runs_per_input;
    let total_runs = base.warmup_runs + total_timed_runs;

    let mut times: Vec<f64> = Vec::with_capacity(total_timed_runs);

    match benchmark {
        "matrix_mul" => {
            let in_dir = format!("{}/matrix_mul", base.input_dir);
            let mut a_list: Vec<Array2<f64>> = Vec::with_capacity(base.num_inputs);
            let mut b_list: Vec<Array2<f64>> = Vec::with_capacity(base.num_inputs);

            for k in 0..base.num_inputs {
                let a_path = format!("{}/a_{:03}.npy", in_dir, k);
                let b_path = format!("{}/b_{:03}.npy", in_dir, k);
                let a: Array2<f64> = read_npy(a_path).unwrap();
                let b: Array2<f64> = read_npy(b_path).unwrap();
                a_list.push(a);
                b_list.push(b);
            }

            for i in 0..total_runs {
                let idx = i % base.num_inputs;
                let start = Instant::now();
                let _ = matrix_mul::run(&a_list[idx], &b_list[idx]);
                let elapsed = start.elapsed().as_secs_f64();
                if i >= base.warmup_runs {
                    times.push(elapsed);
                }
            }

            write_csv_matrix_mul(base, &cfg, &times);
        }

        "reduction" => {
            let in_dir = format!("{}/reduction", base.input_dir);
            let mut x_list: Vec<Array1<f64>> = Vec::with_capacity(base.num_inputs);

            for k in 0..base.num_inputs {
                let x_path = format!("{}/x_{:03}.npy", in_dir, k);
                let x: Array1<f64> = read_npy(x_path).unwrap();
                x_list.push(x);
            }

            for i in 0..total_runs {
                let idx = i % base.num_inputs;
                let start = Instant::now();
                let _ = reduction::run(x_list[idx].as_slice().unwrap());
                let elapsed = start.elapsed().as_secs_f64();
                if i >= base.warmup_runs {
                    times.push(elapsed);
                }
            }

            write_csv_reduction(base, &cfg, &times);
        }

        _ => panic!("Unknown benchmark: {}", benchmark),
    }
}

fn write_csv_matrix_mul(base: &BaseConfig, cfg: &Value, times: &[f64]) {
    let benchmark = "matrix_mul";
    let output_path = format!("{}/rust_{}.csv", base.output_dir, benchmark);
    let mut wtr = Writer::from_path(&output_path).unwrap();

    wtr.write_record([
        "run",
        "time_sec",
        "num_threads",
        "num_inputs",
        "runs_per_input",
        "seed",
        "matrix_size",
    ])
    .unwrap();

    let matrix_size = cfg["matrix_size"].as_i64().unwrap().to_string();

    for (i, t) in times.iter().enumerate() {
        wtr.write_record([
            i.to_string(),
            t.to_string(),
            base.num_threads.to_string(),
            base.num_inputs.to_string(),
            base.runs_per_input.to_string(),
            base.seed.to_string(),
            matrix_size.clone(),
        ])
        .unwrap();
    }

    println!("Results written to {}", output_path);
}

fn write_csv_reduction(base: &BaseConfig, cfg: &Value, times: &[f64]) {
    let benchmark = "reduction";
    let output_path = format!("{}/rust_{}.csv", base.output_dir, benchmark);
    let mut wtr = Writer::from_path(&output_path).unwrap();

    wtr.write_record([
        "run",
        "time_sec",
        "num_threads",
        "num_inputs",
        "runs_per_input",
        "seed",
        "vector_size",
    ])
    .unwrap();

    let vector_size = cfg["vector_size"].as_i64().unwrap().to_string();

    for (i, t) in times.iter().enumerate() {
        wtr.write_record([
            i.to_string(),
            t.to_string(),
            base.num_threads.to_string(),
            base.num_inputs.to_string(),
            base.runs_per_input.to_string(),
            base.seed.to_string(),
            vector_size.clone(),
        ])
        .unwrap();
    }

    println!("Results written to {}", output_path);
}
