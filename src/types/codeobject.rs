use std::ffi::CString;

use crate::ffi;
use crate::{PyAny, PyResult, Python, PyNativeType, AsPyPointer};
use crate::types::{PyBytes, PyTuple};

/// Represents a Python code object
#[repr(transparent)]
#[cfg(not(PyPy))]
pub struct PyCodeObject(PyAny);

pyobject_native_type_core!(PyCodeObject, ffi::PyCode_Type, #checkfunction=ffi::PyCode_Check);

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

    pub fn code (&self) -> &PyBytes {
        unsafe {
            return self.py()
                .from_owned_ptr::<PyBytes>(ffi::PyCode_GetCode(self.as_ptr()));
        }
    }

    pub fn consts(&self) -> &PyTuple {
        unsafe {
            return self.py()
                .from_owned_ptr::<PyTuple>(ffi::PyCode_GetConsts(self.as_ptr()));
        }
    }

    pub fn names(&self) -> &PyTuple {
        unsafe {
            return self.py()
                .from_owned_ptr::<PyTuple>(ffi::PyCode_GetNames(self.as_ptr()));
        }
    }

    pub fn var_names(&self) -> &PyTuple {
        unsafe {
            return self.py()
                .from_owned_ptr::<PyTuple>(ffi::PyCode_GetVarNames(self.as_ptr()));
        }
    }

    pub fn free_vars(&self) -> &PyTuple {
        unsafe {
            return self.py()
                .from_owned_ptr::<PyTuple>(ffi::PyCode_GetFreeVars(self.as_ptr()));
        }
    }

    pub fn cell_vars(&self) -> &PyTuple {
        unsafe {
            return self.py()
                .from_owned_ptr::<PyTuple>(ffi::PyCode_GetCellVars(self.as_ptr()));
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::types::codeobject::PyCodeObject;
    use crate::{Python};

    #[test]
    fn test_compile_string() {
        Python::with_gil(|py| {
            let code_object = PyCodeObject::compile_string(py, "a = 3 + 6", "<filename>")
                .expect("Code compilation failed");

            println!("Consts: {:?}", code_object.consts());
            println!("Names: {:?}", code_object.names());
            println!("Var Names: {:?}", code_object.var_names());
            println!("Cell Vars: {:?}", code_object.cell_vars());
            println!("Code: {:?}", code_object.code());
            println!("Free Vars: {:?}", code_object.free_vars());
        });
    }

} 