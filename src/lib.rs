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

    pub fn add(&mut self, elem: PyObject) -> PyResult<()> {
        self.cvm.process_element(PyObjectWrapper::new(elem));
        Ok(())
    }

    pub fn calculate_final_result(&self) -> f64 {
        self.cvm.calculate_final_result()
    }
}

#[pymodule]
fn cvmcount(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CVM>()?;
    // m.add_function(wrap_pyfunction!(version, m)?)?;

    Ok(())
}
