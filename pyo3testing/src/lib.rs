extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn pyo3test (_attr: TokenStream, _item: TokenStream) -> TokenStream {
    panic!();
    let out = quote! {
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
    };
    out.into()
}

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}