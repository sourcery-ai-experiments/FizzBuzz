#[macro_use]
extern crate pyo3testing;
struct Test;
struct Hello;
pyo3::append_to_inittab!(py_fizzbuzzo3);