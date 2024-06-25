# A Python implementation of the CVM Algorithm for Counting Distinct Elements

This library implements the algorithm described in

> Chakraborty, S., Vinodchandran, N. V., & Meel, K. S. (2022). *Distinct Elements in Streams: An Algorithm for the (Text) Book*. 6 pages, 727571 bytes. https://doi.org/10.4230/LIPIcs.ESA.2022.34

The accompanying article in Quanta is here: https://www.quantamagazine.org/computer-scientists-invent-an-efficient-new-way-to-count-20240516/

This Python module leverages the `cvmcount` Rust library: https://github.com/urschrei/cvmcount

## What does that mean
The count-distinct problem, or cardinality-estimation problem refers to counting the number of distinct elements in a data stream with repeated elements. As a concrete example, imagine that you want to count the unique words in a book. If you have enough memory, you can keep track of every unique element you encounter. However, you may not have enough working memory due to resource constraints, or the number of potential elements may be enormous. This constraint is referred to as the bounded-storage constraint in the literature.

In order to overcome this constraint, streaming algorithms have been developed: [Flajolet-Martin](https://en.wikipedia.org/wiki/Flajolet–Martin_algorithm), LogLog, [HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog). The algorithm implemented by this library is an improvement on these in one particular sense: it is extremely simple. Instead of hashing, it uses a sampling method to compute an [unbiased estimate](https://www.statlect.com/glossary/unbiased-estimator#:~:text=An%20estimator%20of%20a%20given,Examples) of the cardinality.

# What is an Element
Various integer flavours, strings, any Struct on which you have implemented the traits. Not `f32` / `f64`, however, as they don't form a total ordering due to the presence of NaN.

## Further Details
Don Knuth has written about the algorithm (he refers to it as **Algorithm D**) at https://cs.stanford.edu/~knuth/papers/cvm-note.pdf, and does a far better job than I do at explaining it. You will note that on p1 he describes the buffer he uses as a data structure – called a [treap](https://en.wikipedia.org/wiki/Treap#:~:text=7%20External%20links-,Description,(randomly%20chosen)%20numeric%20priority.) – as a binary tree
> "that’s capable of holding up to _s_ ordered pairs (_a_, _u_), where _a_ is an element of the stream and _u_ is a real number, 0 ≤ _u_ < 1."

where _s_ >= 1. Our implementation doesn't use a treap as a buffer; it uses a fast HashSet with the [FxHash](https://docs.rs/fxhash/latest/fxhash/) algorithm: we pay the hash cost when inserting, but search in step **D4** is `O(1)`. The library may switch to a treap implementation eventually.

# Installation
`pip install count_distinct`

# Usage
```python
from count_distinct import CVM

# values for epsilon, delta, and stream size are described in the docstring.
counter = CVM(0.8, 0.1, 1000)
counter.add(2)
counter.add(5)
# keep adding elements
count = counter.calculate_final_result()
# you can keep adding elements if you wish
```

## Note
If you're thinking about using this library, you presumably know that it only provides an estimate (within the specified bounds), similar to something like HyperLogLog. You are trading accuracy for speed!
