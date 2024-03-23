#[test]
fn test_fizzbuzz() {
    pyo3::append_to_inittab!(py_fizzbuzzo3);
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fizzbuzzo3 = py
            .import_bound("fizzbuzzo3")
            .expect("Failed to import fizzbuzzo3");
        let fizzbuzz = fizzbuzzo3
            .getattr("fizzbuzz")
            .expect("Failed to get fizzbuzz function");
        let result: PyResult<String> = match fizzbuzz.call1((1i32,)) {
            Ok(r) => r.extract(),
            Err(e) => Err(e),
        };
        let result = result.unwrap();
        let expected_result = "1";
        assert_eq!(result, expected_result);
    });
}