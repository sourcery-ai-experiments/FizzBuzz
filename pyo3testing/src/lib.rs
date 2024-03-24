extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use pyo3;

#[proc_macro_attribute]
pub fn pyo3test(_args: TokenStream, input: TokenStream) -> TokenStream {
    
    let input = parse_macro_input!(input as DeriveInput);
    
    let tokens = quote! {
        #input
    };

    tokens.into()
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
        pyo3::prepare_freethreaded_python();
        ::pyo3::append_to_inittab!(py_fizzbuzzo3);
    };

    tokens.into()
}