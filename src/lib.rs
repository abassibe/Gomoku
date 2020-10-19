mod goban;

use numpy::{PyArray2};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
use pyo3::{IntoPy, exceptions};
use pyo3::types::{PyTuple, PyBool, PyInt, PyLong};
use goban::Goban;

const EMPTY: u8 = 0;
const WHITE: u8 = 2;
const BLACK: u8 = 1;
const BWIDTH: u8 = 19;
const BHEIGHT: u8 = 19;
const BSIZE: u16 = 361;

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "ret_coord")]
    fn ret_coord(py: Python<'_>, goban: &PyArray2<u8>, p_color: u8, hint: &PyBool) -> PyResult<Py<PyTuple>> {
        let board = goban.to_vec()?;
        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }
        let goban: Goban = Goban::new(board, p_color);
        // Do my stuff
        let ret: &PyTuple = PyTuple::new(py, [2, 1].iter()); //placeholder
        Ok(ret.into_py(py))
    }
    #[pyfn(m, "debug_heuristic")]
    fn debug_heuristic(py: Python<'_>, goban: &PyArray2<u8>) -> PyResult<u32> {
        let board = goban.to_vec()?;
        let mut ret: u32 = 0;
        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }
        ret = 33;
        Ok(ret)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
