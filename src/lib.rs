mod islandmystic;
mod symol;
mod php5random;
use pyo3::prelude::*;

use islandmystic::IslandMystic;
use symol::Symol;

#[pymodule]
fn rust_neotools(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<IslandMystic>()?;
    m.add_class::<Symol>()?;

    Ok(())
}
