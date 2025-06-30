import numpy as np
from count_distinct import CVM


def test_cvmcount() -> None:
    counter = CVM(0.8, 0.1, 3)
    counter.add(1)
    counter.add(2)
    counter.add(2)
    counter.add(2)
    counter.add(2)
    counter.add(2)
    counter.add(2)
    counter.add(2)
    counter.add(2)
    counter.add(3)
    counter.add(4)
    counter.add(4)
    counter.add(4)
    counter.add(4)
    counter.add(4)
    counter.add(4)
    counter.add(4)
    counter.add(4)
    counter.add(4)
    count = counter.calculate_final_result()
    assert count == 4
