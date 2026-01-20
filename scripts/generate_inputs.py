import os
import yaml
import numpy as np


def load_yaml(path):
    with open(path, "r") as f:
        return yaml.safe_load(f)


def ensure_dir(path):
    os.makedirs(path, exist_ok=True)


def generate_matrix_mul_inputs(out_dir, num_inputs, matrix_size, seed):
    ensure_dir(out_dir)
    for k in range(num_inputs):
        rng = np.random.default_rng(seed + k)
        a = rng.random((matrix_size, matrix_size), dtype=np.float64)
        b = rng.random((matrix_size, matrix_size), dtype=np.float64)
        np.save(os.path.join(out_dir, f"a_{k:03d}.npy"), a)
        np.save(os.path.join(out_dir, f"b_{k:03d}.npy"), b)


def generate_reduction_inputs(out_dir, num_inputs, vector_size, seed):
    ensure_dir(out_dir)
    for k in range(num_inputs):
        rng = np.random.default_rng(seed + k)
        x = rng.random((vector_size,), dtype=np.float64)
        np.save(os.path.join(out_dir, f"x_{k:03d}.npy"), x)


def main():
    base = load_yaml("configs/base.yaml")

    input_dir = base["input_dir"]
    num_inputs = int(base["num_inputs"])
    seed = int(base["seed"])

    # Benchmark-specific configs
    mm = load_yaml("configs/matrix_mul.yaml")
    rd = load_yaml("configs/reduction.yaml")

    mm_out = os.path.join(input_dir, "matrix_mul")
    rd_out = os.path.join(input_dir, "reduction")

    print(f"Generating {num_inputs} matrix_mul inputs into: {mm_out}")
    generate_matrix_mul_inputs(
        out_dir=mm_out,
        num_inputs=num_inputs,
        matrix_size=int(mm["matrix_size"]),
        seed=seed,
    )

    print(f"Generating {num_inputs} reduction inputs into: {rd_out}")
    generate_reduction_inputs(
        out_dir=rd_out,
        num_inputs=num_inputs,
        vector_size=int(rd["vector_size"]),
        seed=seed,
    )

    print("Done.")


if __name__ == "__main__":
    main()
