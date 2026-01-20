#!/bin/bash
# Make sure script stops if a command fails
set -e

echo "Running all Python benchmarks..."
python3 python/run_benchmark.py configs/matrix_mul.yaml
python3 python/run_benchmark.py configs/reduction.yaml

echo "Running all Rust benchmarks..."
cargo run --release -- configs/matrix_mul.yaml
cargo run --release -- configs/reduction.yaml

echo "All benchmarks finished. CSV results are in the 'results/' folder."
