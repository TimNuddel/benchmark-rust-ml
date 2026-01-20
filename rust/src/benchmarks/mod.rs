pub mod matrix_mul;
pub mod reduction;

use csv::Writer;
use serde_yaml::Value;
use std::fs::{File, create_dir_all};
use std::time::Instant;

use crate::BaseConfig;

pub fn dispatch(config_path: &str, base: &BaseConfig) {
    let config: Value =
        serde_yaml::from_reader(File::open(config_path).unwrap()).unwrap();

    let benchmark = config["benchmark"].as_str().unwrap();
    let mut times = Vec::new();

    // Ensure results folder exists
    create_dir_all(&base.output_dir).unwrap();

    for i in 0..(base.runs + base.warmup_runs) {
        let start = Instant::now();

        match benchmark {
            "matrix_mul" => matrix_mul::run(&config),
            "reduction" => reduction::run(&config),
            _ => panic!("Unknown benchmark"),
        }

        let elapsed = start.elapsed().as_secs_f64();
        if i >= base.warmup_runs {
            times.push(elapsed);
        }
    }

    // CSV output
    let output_path = format!("{}/rust_{}.csv", base.output_dir, benchmark);
    let mut wtr = Writer::from_path(&output_path).unwrap();

    // Headers
    let mut headers = vec!["run", "time_sec", "num_threads"];
    if benchmark == "matrix_mul" {
        headers.push("matrix_size");
    } else if benchmark == "reduction" {
        headers.push("vector_size");
    }
    wtr.write_record(&headers).unwrap();

    // Data
    for (i, t) in times.iter().enumerate() {
        let mut row = vec![i.to_string(), t.to_string(), base.num_threads.to_string()];
        if benchmark == "matrix_mul" {
            row.push(config["matrix_size"].as_i64().unwrap().to_string());
        } else if benchmark == "reduction" {
            row.push(config["vector_size"].as_i64().unwrap().to_string());
        }
        wtr.write_record(&row).unwrap();
    }

    println!("Results written to {}", output_path);
}
