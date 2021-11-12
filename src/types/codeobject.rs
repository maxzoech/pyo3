use std::ffi::CString;

use crate::ffi;
use crate::{PyAny, PyResult, Python};

/// Represents a Python code object
#[repr(transparent)]
pub struct PyCodeObject(PyAny);

pyobject_native_type_core!(PyCodeObject, ffi::PyTuple_Type, #checkfunction=ffi::PyCode_Check);

impl PyCodeObject {

    pub fn compile_string<'a>(py: Python<'a>, string: &str, filename: &str) -> PyResult<&'a PyCodeObject> {
        let code = CString::new(string).unwrap();
        let filename = CString::new(filename).unwrap();

        unsafe {
            return py.from_borrowed_ptr_or_err::<PyCodeObject>(
                ffi::Py_CompileString(code.as_ptr(), filename.as_ptr(), ffi::Py_file_input)
            );
        }
    }

    pub fn code (&self) {
        
    }
}

#[cfg(test)]
mod tests {
    use crate::types::codeobject::PyCodeObject;
    use crate::{Python};

    #[test]
    fn test_compile_string() {
        Python::with_gil(|py| {
            PyCodeObject::compile_string(py, "a = 3 + 6", "<filename>")
                .expect("Code compilation failed");
        });
    }

} 