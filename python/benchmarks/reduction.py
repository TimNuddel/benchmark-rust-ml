import numpy as np

def run_reduction(config):
    n = config["vector_size"]
    x = np.random.rand(n)
    _ = x.sum()
