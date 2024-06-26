import numpy as np
import pyperf
from count_distinct import CVM


def add_elems(counter, elems):
    for elem in elems:
        counter.add(elem)


# Generate 10 million random 7-digit integers
random_integers = np.random.randint(low=1000000, high=10000000, size=100_000_000)

counter = CVM(0.8, 0.1, 100000)

add_elems(counter, random_integers)
count = counter.calculate_final_result()
