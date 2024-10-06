use pyo3::prelude::*;

pub fn my_module(py: Python<'_>)  -> PyResult<()> {
    // Use include_str! to embed the Python file's content at build time
    let py_code = include_str!("/app/src-py/from_rust.py");

    // Execute the Python code as a module
    let activators = PyModule::from_code_bound(py, py_code, "from_rust.py", "from_rust").unwrap();
    
    // Call the Python function
    let result: f64 = activators
        .getattr("my_numpy_function")
        .unwrap()
        .call1((5.0,))
        .unwrap()
        .extract()
        .unwrap();

    println!("Result from Python function: {}", result);
    Ok(())
}

pub fn numpy(py: Python<'_>) -> PyResult<()> {
    let numpy = PyModule::import_bound(py, "numpy").unwrap();
    let arr = numpy.call_method1("array", (vec![1.0, 2.0, 3.0],)).unwrap();
    println!("Array from NumPy: {:?}", arr);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_import() {
        Python::with_gil(|py| {
            let _ = my_module(py);
            let _ = numpy(py);
        });
    }
}
