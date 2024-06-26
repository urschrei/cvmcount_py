import numpy as np
import memray

def add_elems(elems):
    return np.unique(elems)

# Generate 100 million random 7-digit integers
random_integers = np.random.randint(low=1000000, high=10000000, size=100_000_000)

res = add_elems(random_integers)
