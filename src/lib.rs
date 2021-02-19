use numpy::PyArray2;
use pyo3::exceptions;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::types::PyBool;

use goban::Goban;

use crate::algorithm::Algorithm;
use crate::bitboard::BitBoard;
use crate::node::Node;

mod goban;
mod bitboard;
mod stone;
mod node;
mod algorithm;

// Comment rajouter une fonction python sur rust
// Simplement rajouter dans le block pymodule une fonction rust avec obligatoirement une instance Python<'_>, et si applicable un PyResult pour le retour
// Presque n'importe quel type peut etre passé tant que c'est un type natif python/rust (check doc)
// Pour compiler, maturin develop dans le terminal, qui genere un dylib dans le dossier target/debug qu'il faut mettre dans le dossier root du projet.

const DEPTH: u32 = 6;
const WHITE: u8 = 2;
const BLACK: u8 = 1;

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {


    #[pyfn(m, "get_next_move")]
    fn get_next_move(py: Python<'_>, goban: &PyArray2<u8>, p_color: u8, hint: &PyBool, human_capture: i32, ai_capture: i32) -> PyResult<(u32, u32)> {
        let board:Vec<u8> = goban.to_vec()?;

        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!("Fatal Rust Error: Invalid board size (Expected 361, got {})", board.len())));
        }

        let goban = assign_color_to_ai(vec_to_string(board), p_color);
        println!("\nCOLOR IS ={}\n\nPLAYER(AI)\n{}\nENEMY\n{}", p_color, goban.get_player(), goban.get_enemy());

        if goban.get_board().is_empty() {
            return Ok((9u32, 9u32));
        }
        let ret = launch_ai(goban, (ai_capture / 2) as u8, (human_capture / 2) as u8);
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

fn assign_color_to_ai(str: String, human: u8) -> Goban {
    let player = BitBoard::from_str(&str.replace("2", "0"));
    let enemy = BitBoard::from_str(&str.replace("1", "0").replace("2", "1"));

    if human == WHITE {
        Goban::new(player, enemy)
    }
    else {
        Goban::new(enemy, player)
    }
}

fn launch_ai(input: Goban, player_captures: u8, opponent_captures: u8) -> (u32, u32) {
    let mut algorithm = Algorithm::new();
    let ret_node = Node::default();
    algorithm.update_initial_state(input, *input.get_enemy(), player_captures, opponent_captures);
    let ret = algorithm.get_next_move(DEPTH).unwrap();

    get_win_coord(*input.get_player(), *ret.get_item().get_player())
}

fn get_win_coord(previous: BitBoard, current: BitBoard) -> (u32, u32) {
    let pos = previous ^ current;
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
