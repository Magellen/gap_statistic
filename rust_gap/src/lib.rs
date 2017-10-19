#![feature(proc_macro, specialization)]

extern crate pyo3;
use pyo3::prelude::*;


#[py::modinit(rust_gap)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "sum_as_string")]
    fn sum_as_string_py(a: i64, b: i64) -> PyResult<String> {
        let out = sum_as_string(a, b);
        Ok(out)
    }
    Ok(())
}

fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b).to_string()
}