#[macro_use]
extern crate pyo3test;

#[pyo3test(fizzbuzzo3, fizzbuzz)]
fn test_fizzbuzz() {
    let result: PyResult<String> = match fizzbuzz.call1((1i32,)) {
        Ok(r) => r.extract(),
        Err(e) => Err(e),
    };
    let result = result.unwrap();
    let expected_result = "1";
    assert_eq!(result, expected_result);
}