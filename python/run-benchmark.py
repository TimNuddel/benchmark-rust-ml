import os
import sys
import yaml
import time
import csv
import numpy as np

from benchmarks.matrix_mul import run_matrix_mul
from benchmarks.reduction import run_reduction


def set_thread_limits(n: int):
    os.environ["OMP_NUM_THREADS"] = str(n)
    os.environ["MKL_NUM_THREADS"] = str(n)
    os.environ["OPENBLAS_NUM_THREADS"] = str(n)


def load_yaml(path: str):
    with open(path, "r") as f:
        return yaml.safe_load(f)


def load_inputs_matrix_mul(input_dir: str, num_inputs: int):
    a_list, b_list = [], []
    for k in range(num_inputs):
        a = np.load(os.path.join(input_dir, f"a_{k:03d}.npy"))
        b = np.load(os.path.join(input_dir, f"b_{k:03d}.npy"))
        a_list.append(a)
        b_list.append(b)
    return a_list, b_list


def load_inputs_reduction(input_dir: str, num_inputs: int):
    x_list = []
    for k in range(num_inputs):
        x = np.load(os.path.join(input_dir, f"x_{k:03d}.npy"))
        x_list.append(x)
    return x_list


def main(config_path: str):
    base = load_yaml("configs/base.yaml")
    cfg = load_yaml(config_path)
    config = {**base, **cfg}

    os.makedirs(config["output_dir"], exist_ok=True)
    set_thread_limits(int(config["num_threads"]))

    benchmark = config["benchmark"]
    num_inputs = int(config["num_inputs"])
    runs_per_input = int(config["runs_per_input"])
    warmup_runs = int(config["warmup_runs"])
    total_timed_runs = num_inputs * runs_per_input

    # Preload inputs (NOT timed)
    if benchmark == "matrix_mul":
        in_dir = os.path.join(config["input_dir"], "matrix_mul")
        a_list, b_list = load_inputs_matrix_mul(in_dir, num_inputs)
    elif benchmark == "reduction":
        in_dir = os.path.join(config["input_dir"], "reduction")
        x_list = load_inputs_reduction(in_dir, num_inputs)
    else:
        raise ValueError(f"Unknown benchmark: {benchmark}")

    times = []

    for i in range(warmup_runs + total_timed_runs):
        input_idx = i % num_inputs

        start = time.perf_counter()
        if benchmark == "matrix_mul":
            _ = run_matrix_mul(a_list[input_idx], b_list[input_idx])
        else:
            _ = run_reduction(x_list[input_idx])
        elapsed = time.perf_counter() - start

        if i >= warmup_runs:
            times.append(elapsed)

    output_path = os.path.join(config["output_dir"], f"python_{benchmark}.csv")
    with open(output_path, "w", newline="") as f:
        w = csv.writer(f)

        # headers
        headers = [
            "run",
            "time_sec",
            "num_threads",
            "num_inputs",
            "runs_per_input",
            "seed",
        ]
        if benchmark == "matrix_mul":
            headers.append("matrix_size")
        else:
            headers.append("vector_size")
        w.writerow(headers)

        # rows
        for run_id, t in enumerate(times):
            row = [
                run_id,
                t,
                config["num_threads"],
                config["num_inputs"],
                config["runs_per_input"],
                config["seed"],
            ]
            if benchmark == "matrix_mul":
                row.append(config["matrix_size"])
            else:
                row.append(config["vector_size"])
            w.writerow(row)

    print(f"Results written to {output_path}")


if __name__ == "__main__":
    main(sys.argv[1])
