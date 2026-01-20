# Rust vs Python Benchmarking for ML/AI

## Overview

This repository contains a preliminary benchmarking framework to compare **Python** and **Rust** for basic numerical workloads, as a foundation for exploring Rust’s future role in Machine Learning and AI.  

The goals of this repository are:

- Provide **reproducible benchmarks** for simple tasks.
- Support multiple benchmarks with **shared configuration**.
- Automatically save results in **self-describing CSV files**.
- Allow both Python and Rust implementations to run on the same machine under controlled thread settings.

---

## Repository Structure

```
benchmark-rust-ml/
├── README.md
├── .gitignore
├── requirements.txt
├── configs/
│   ├── base.yaml
│   ├── matrix_mul.yaml
│   └── reduction.yaml
├── python/
│   ├── run_benchmark.py
│   └── benchmarks/
│       ├── matrix_mul.py
│       └── reduction.py
├── rust/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── benchmarks/
│           ├── matrix_mul.rs
│           └── reduction.rs
├── results/
│   └── .gitkeep
└── scripts/
    └── run_all.sh
```

- **configs/** – YAML files with base settings and benchmark-specific parameters.
- **python/** – Python benchmark code and runner.
- **rust/** – Rust benchmark code.
- **results/** – CSV output from benchmark runs.
- **scripts/run_all.sh** – Run all Python and Rust benchmarks automatically.

---

## Installation

### 1. Python Environment

We recommend **Python 3.11.8** for reproducibility.

#### Using venv:

```bash
python3.11 -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
```

#### Using Conda:

```bash
conda create -n rust-ml-bench python=3.11.8 numpy pyyaml
conda activate rust-ml-bench
```

### 2. Rust Environment

We recommend **Rust 1.72.0 (stable)**:

```bash
rustup install 1.72.0
rustup default 1.72.0
rustc --version
cargo --version
```

---

## Running Benchmarks

### 1. Make the script executable (Linux/macOS only)

```bash
chmod +x scripts/run_all.sh
```

### 2. Run all benchmarks

```bash
./scripts/run_all.sh
```

This will:

- Run all Python benchmarks defined in `configs/`
- Run all Rust benchmarks defined in `configs/`
- Save results to `results/` as CSV files:

```
results/python_matrix_mul.csv
results/python_reduction.csv
results/rust_matrix_mul.csv
results/rust_reduction.csv
```

---

## Benchmark Configuration

- **configs/base.yaml** – shared defaults:

```yaml
runs: 20
warmup_runs: 5
num_threads: 4
output_dir: results
```

- **configs/matrix_mul.yaml** – matrix multiplication benchmark:

```yaml
benchmark: matrix_mul
matrix_size: 1024
```

- **configs/reduction.yaml** – vector reduction benchmark:

```yaml
benchmark: reduction
vector_size: 10000000
```

> You can add new benchmarks by creating new config files and corresponding Python/Rust benchmark modules.

---

## CSV Output

Each CSV file contains:

| Column       | Description |
|-------------|------------|
| run         | Run index (0-based) |
| time_sec    | Execution time in seconds |
| num_threads | Number of threads used |
| matrix_size / vector_size | Benchmark-specific parameter |

This format ensures **self-contained reproducibility**.

---

## Notes

- Benchmark tasks are CPU-only and deterministic.
- Randomness is introduced only in input arrays to simulate real workloads.
- `results/` folder is automatically created if missing.
- Thread count is enforced for consistent performance measurement.
- External datasets or models are **not included**; instructions for downloading and integrating them will be added as benchmarks grow.

---

## License

This repository is released under the **MIT License**. See [LICENSE](LICENSE) for details.

---

## Contact / Questions

For questions about this benchmarking framework, please contact the author (or include your email here if desired).

