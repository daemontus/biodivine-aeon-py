use super::PyParameterId;
use crate::throw_runtime_error;
use biodivine_lib_param_bn::ParameterId;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl From<PyParameterId> for ParameterId {
    fn from(value: PyParameterId) -> Self {
        value.0
    }
}

impl From<ParameterId> for PyParameterId {
    fn from(value: ParameterId) -> Self {
        PyParameterId(value)
    }
}

#[pymethods]
impl PyParameterId {
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => throw_runtime_error("Unsupported operation."),
            CompareOp::Le => throw_runtime_error("Unsupported operation."),
            CompareOp::Eq => Ok(self == other),
            CompareOp::Ne => Ok(self != other),
            CompareOp::Gt => throw_runtime_error("Unsupported operation."),
            CompareOp::Ge => throw_runtime_error("Unsupported operation."),
        }
    }

    fn __hash__(&self) -> isize {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as isize
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    #[staticmethod]
    pub fn from_index(value: usize) -> PyParameterId {
        ParameterId::from_index(value).into()
    }

    pub fn as_index(&self) -> usize {
        self.0.to_index()
    }
}
