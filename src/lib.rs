use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn test_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::Local;

    use super::*;

    #[test]
    fn it_works() {
        let local = Local::now().timestamp_millis();
        let result = sum_as_string(2, 2);
        println!("------sum_as_string:{}", result.expect(""));
        println!("time ms:{}", Local::now().timestamp_millis() - local);

        // assert_eq!(result, 4);
    }
}
