use ::cvmcount::CVM as RustCVM;
use pyo3::prelude::*;
use std::hash::{Hash, Hasher};

/// A wrapper for `PyObject` to implement `Hash` and `Eq`.
#[derive(Clone)]
pub struct PyObjectWrapper(PyObject);

impl PyObjectWrapper {
    fn new(obj: PyObject) -> Self {
        PyObjectWrapper(obj)
    }
}

impl Hash for PyObjectWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

impl PartialEq for PyObjectWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl Eq for PyObjectWrapper {}

#[pyclass]
/// Initialise the algorithm
///
/// epsilon: how close you want your estimate to be to the true number of distinct elements.
/// A smaller ε means you require a more precise estimate.
/// For example, ε = 0.05 means you want your estimate to be within 5% of the actual value.
/// An epsilon of 0.8 is a good starting point for most applications.
///
/// delta: The level of certainty that the algorithm's estimate will fall within the desired accuracy range. A higher confidence
/// (e.g. 99.9 %) means you're very sure the estimate will be accurate, while a lower confidence (e.g. 90 %) means there's a
/// higher chance the estimate might be outside the desired range.
/// A delta of 0.1 is a good starting point for most applications.
///
/// stream_size: this is used to determine buffer size and can be a loose approximation. The closer it is to the stream size,
/// the more accurate the result will be
struct CVM {
    cvm: RustCVM<PyObjectWrapper>,
}

#[pymethods]
impl CVM {
    #[new]
    pub fn new(epsilon: f64, delta: f64, stream_size: usize) -> Self {
        CVM {
            cvm: RustCVM::new(epsilon, delta, stream_size),
        }
    }

    /// Add an element, potentially updating the unique element count
    pub fn add(&mut self, elem: PyObject) -> PyResult<()> {
        self.cvm.process_element(PyObjectWrapper::new(elem));
        Ok(())
    }

    /// Calculate the current unique element count. You can continue to add elements after calling this method
    pub fn calculate_final_result(&self) -> f64 {
        self.cvm.calculate_final_result()
    }
}

#[pymodule]
fn cvmcount(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CVM>()?;
    // m.add_function(wrap_pyfunction!(version, m)?)?;

    Ok(())
}
