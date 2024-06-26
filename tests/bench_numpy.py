import numpy as np
import pyperf

def add_elems(elems):
    return np.unique(elems)


# Generate 10 million random 7-digit integers
random_integers = np.random.randint(low=1000000, high=10000000, size=10_000_000)

runner = pyperf.Runner()
runner.bench_func(
    "10 million 7-digit random positive integers: numpy",
    add_elems, random_integers
)

