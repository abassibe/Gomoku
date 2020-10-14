use numpy::{IntoPyArray, PyArray2, PyArray1};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
use pyo3::IntoPy;

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "read_board")]
    fn read_board(py: Python<'_>, x: &PyArray2<i32>) -> PyResult<Py<PyArray2<i32>>> {
        let board = x.to_vec().unwrap();
        let test = board.clone().into_pyarray(py);
        // Do my stuff
	    let ret : &PyArray2<i32> = PyArray1::from_vec(py, board).reshape([19,19]).unwrap();
        Ok(ret.into_py(py))
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
