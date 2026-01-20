import os
import sys
import yaml
import time
import csv

from benchmarks.matrix_mul import run_matrix_mul
from benchmarks.reduction import run_reduction


def set_thread_limits(n):
    os.environ["OMP_NUM_THREADS"] = str(n)
    os.environ["MKL_NUM_THREADS"] = str(n)
    os.environ["OPENBLAS_NUM_THREADS"] = str(n)


def main(config_path):
    # Load benchmark-specific config
    with open(config_path) as f:
        config = yaml.safe_load(f)

    # Load base config
    with open("configs/base.yaml") as f:
        base = yaml.safe_load(f)

    # Merge configs (benchmark overrides base)
    config = {**base, **config}

    # Ensure results folder exists
    os.makedirs(config["output_dir"], exist_ok=True)

    # Set thread limits
    set_thread_limits(config["num_threads"])

    benchmark = config["benchmark"]
    results = []

    for run_id in range(config["runs"] + config["warmup_runs"]):
        start = time.perf_counter()

        if benchmark == "matrix_mul":
            run_matrix_mul(config)
        elif benchmark == "reduction":
            run_reduction(config)
        else:
            raise ValueError("Unknown benchmark")

        elapsed = time.perf_counter() - start

        if run_id >= config["warmup_runs"]:
            results.append(elapsed)

    # CSV output with extra metadata
    output_path = f'{config["output_dir"]}/python_{benchmark}.csv'
    with open(output_path, "w", newline="") as f:
        writer = csv.writer(f)
        # Write headers
        headers = ["run", "time_sec", "num_threads"]
        # Add benchmark-specific params
        if benchmark == "matrix_mul":
            headers.append("matrix_size")
        elif benchmark == "reduction":
            headers.append("vector_size")
        writer.writerow(headers)

        # Write data
        for i, t in enumerate(results):
            row = [i, t, config["num_threads"]]
            if benchmark == "matrix_mul":
                row.append(config["matrix_size"])
            elif benchmark == "reduction":
                row.append(config["vector_size"])
            writer.writerow(row)

    print(f"Results written to {output_path}")


if __name__ == "__main__":
    main(sys.argv[1])
