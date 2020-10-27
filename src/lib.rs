mod goban;

use numpy::{PyArray2};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
use pyo3::{exceptions};
use pyo3::types::{PyBool};
use goban::Goban;

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "ret_coord")]
    fn ret_coord(py: Python<'_>, goban: &PyArray2<u8>, p_color: u8, hint: &PyBool) -> PyResult<(u32, u32)> {
        let board = goban.to_vec()?;
        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }
        let goban: Goban = Goban::new(board, p_color);
        // Do my stuff
        let ret = (5, 6); //placeholder
        Ok(ret)
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
