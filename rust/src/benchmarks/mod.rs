pub mod matrix_mul;
pub mod reduction;

use csv::Writer;
use serde_yaml::Value;
use std::fs::File;
use std::time::Instant;

use crate::BaseConfig;

pub fn dispatch(config_path: &str, base: &BaseConfig) {
    let config: Value =
        serde_yaml::from_reader(File::open(config_path).unwrap()).unwrap();

    let benchmark = config["benchmark"].as_str().unwrap();
    let mut times = Vec::new();

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

    let output_path = format!("{}/rust_{}.csv", base.output_dir, benchmark);
    let mut wtr = Writer::from_path(output_path).unwrap();
    wtr.write_record(&["run", "time_sec"]).unwrap();

    for (i, t) in times.iter().enumerate() {
        wtr.write_record(&[i.to_string(), t.to_string()]).unwrap();
    }
}
