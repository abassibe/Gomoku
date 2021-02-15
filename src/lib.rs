mod goban;
mod bitboard;
mod stone;
mod node;
mod algorithm;

use numpy::{PyArray2};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
use pyo3::{exceptions, PyAny};
use pyo3::types::{PyBool};
use goban::Goban;
use crate::bitboard::BitBoard;

// Comment rajouter une fonction python sur rust
// Simplement rajouter dans le block pymodule une fonction rust avec obligatoirement une instance Python<'_>, et si applicable un PyResult pour le retour
// Presque n'importe quel type peut etre passé tant que c'est un type natif python/rust (check doc)
// Pour compiler, maturin develop dans le terminal, qui genere un dylib dans le dossier target/debug qu'il faut mettre dans le meme dossier que gomoku.py
#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "ret_coord")]
    fn ret_coord(py: Python<'_>, goban: &PyArray2<u8>, p_color: u8, hint: &PyBool) -> PyResult<(u32, u32)> {
        let board:Vec<u8> = goban.to_vec()?;
        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }
        // Do my stuff
        let mut strBoard: String = String::from_utf8(board).unwrap();
        println!("{}", strBoard);
        for i in 0..19 {
            strBoard.insert((i + 1) * 19, '\n');
        }
        println!("{}", strBoard);
        let ret = (5, 6); //placeholder
        Ok(ret)
    }
    #[pyfn(m, "debug")]
    fn debug(py: Python<'_>, goban: &PyArray2<u8>) -> PyResult<u8> {
        let board = goban.to_vec()?;
        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }

        let mut str_board: String = board.into_iter().enumerate().map(|(x, i)|{
            if x % 19 == 0 {
                "\n".to_owned() + &i.to_string()
            }
            else {
                i.to_string()
            }
        }).collect::<String>();
        println!("{}", BitBoard::from_str(&str_board));
        Ok(1u8)
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
