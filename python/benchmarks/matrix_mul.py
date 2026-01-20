import numpy as np

def run_matrix_mul(config):
    n = config["matrix_size"]
    a = np.random.rand(n, n)
    b = np.random.rand(n, n)
    _ = a @ b
