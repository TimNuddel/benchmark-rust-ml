import numpy as np

def run_matrix_mul(a: np.ndarray, b: np.ndarray) -> float:
    c = a @ b
    # Return a tiny scalar so nothing can be optimized away
    return float(c[0, 0])
