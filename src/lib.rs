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
use crate::algorithm::Algorithm;
use crate::node::Node;

// Comment rajouter une fonction python sur rust
// Simplement rajouter dans le block pymodule une fonction rust avec obligatoirement une instance Python<'_>, et si applicable un PyResult pour le retour
// Presque n'importe quel type peut etre passé tant que c'est un type natif python/rust (check doc)
// Pour compiler, maturin develop dans le terminal, qui genere un dylib dans le dossier target/debug qu'il faut mettre dans le meme dossier que gomoku.py
#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {


    #[pyfn(m, "get_next_move")]
    fn get_next_move(py: Python<'_>, goban: &PyArray2<u8>, p_color: u8, hint: &PyBool) -> PyResult<(u32, u32)> {
        let board:Vec<u8> = goban.to_vec()?;

        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }

        let goban = pystring_to_goban(vec_to_string(board), p_color);
        println!("\nCOLOR IS ={}\n\nPLAYER(AI)\n{}\nENEMY\n{}", p_color, goban.get_player(), goban.get_enemy());

        if goban.board.is_empty() {
            return Ok((9u32, 9u32));
        }
        let ret = launch_ai(goban);
        Ok(ret)
    }



    #[pyfn(m, "debug")]
    fn debug(py: Python<'_>, goban: &PyArray2<u8>) -> PyResult<u8> {
        let board = goban.to_vec()?;
        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }
        Ok(1u8)
    }
    Ok(())
}

fn vec_to_string(board: Vec<u8>) -> String {

    board.into_iter().enumerate().map(|(x, i)| {
        if x % 19 == 0 {
            "\n".to_owned() + &i.to_string()
        } else {
            i.to_string()
        }
    }).collect::<String>()
}

fn pystring_to_goban(str: String, p_color: u8) -> Goban {
    if p_color == 1 {
        let player = BitBoard::from_str(&str.replace("2", "0"));
        let enemy = BitBoard::from_str(&str.replace("1", "0").replace("2", "1"));
        Goban::new(player, enemy)
    }
    else {
        let player = BitBoard::from_str(&str.replace("1", "0").replace("2", "1"));
        let enemy = BitBoard::from_str(&str.replace("2", "0"));
        Goban::new(player, enemy)
    }
}

fn launch_ai(input: Goban) -> (u32, u32) {
    let mut algorithm = Algorithm::new();
    let mut ret_node = Node::default();
    algorithm.update_initial_state(input, *input.get_enemy(), ret_node.get_player_captures(), ret_node.get_opponent_captures());
    let ret = algorithm.get_next_move(5).unwrap();

    get_win_coord(input.board, ret.get_item().board)
}

fn get_win_coord(previous: BitBoard, current: BitBoard) -> (u32, u32) {
    let mut pos = previous ^ current;
    println!("PREV:\n{}\nCUR:\n{}", previous, current);
    println!("UNIQUE POS:\n{}", pos);

    let i : u32 = *pos.get_bit_indexes().last().unwrap() as u32;
    println!("I is = {}, coord = {:?}", i, (i / 20, i % 20));
    // println!("{}", pos);
    (i / 20, i % 20)
}

#[cfg(test)]
mod tests {
}
