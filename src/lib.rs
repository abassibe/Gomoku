use node::Node;
use numpy::PyArray2;
use pyo3::exceptions;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::types::PyBool;

use goban::Goban;

use crate::algorithm::Algorithm;
use crate::bitboard::BitBoard;

mod algorithm;
mod bitboard;
mod goban;
mod node;

// Comment rajouter une fonction python sur rust
// Simplement rajouter dans le block pymodule une fonction rust avec obligatoirement une instance Python<'_>, et si applicable un PyResult pour le retour
// Presque n'importe quel type peut etre passé tant que c'est un type natif python/rust (check doc)
// Pour compiler, maturin develop dans le terminal, qui genere un dylib dans le dossier target/debug qu'il faut mettre dans le dossier root du projet.

const DEPTH: u32 = 5;
const WHITE: u8 = 2;
const BLACK: u8 = 1;

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "get_next_move")]
    /// Interfacing function.
    /// Takes the Python GIL, the board in the shape of a 19*19 numpy 2d array, the color of the human player, a boolean that indicates if this is a hint request, and the number of captures made by the human and the ai.
    fn get_next_move(
        py: Python<'_>,
        goban: &PyArray2<u8>,
        p_color: u8,
        hint: &PyBool,
        human_capture: i32,
        ai_capture: i32
    ) -> PyResult<(u32, u32)> {
        let board: Vec<u8> = goban.to_vec()?;

        if board.len() != 361 {
            return Err(exceptions::PyTypeError::new_err(format!(
                "Fatal Rust Error: Invalid board size (Expected 361, got {})",
                board.len()
            )));
        }

        let goban = assign_color_to_ai(vec_to_string(board), p_color);

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
            return Err(exceptions::PyTypeError::new_err(format!(
                "Fatal Rust Error: Invalid board size (Expected 361, got {})",
                board.len()
            )));
        }
        Ok(1u8)
    }
    Ok(())
}

/// Turns the PyArray sent by python into a string that can be turned into a bitboard.
fn vec_to_string(board: Vec<u8>) -> String {
    board
        .into_iter()
        .enumerate()
        .map(|(x, i)| {
            if x % 19 == 0 {
                "\n".to_owned() + &i.to_string()
            } else {
                i.to_string()
            }
        })
        .collect::<String>()
}

/// Turns the string into two bitboards (player, enemy)
fn assign_color_to_ai(str: String, human: u8) -> Goban {
    let player = BitBoard::from_str(&str.replace("2", "0"));
    let enemy = BitBoard::from_str(&str.replace("1", "0").replace("2", "1"));

    if human == WHITE {
        Goban::new(player, enemy)
    } else {
        println!("Goban after color assign : \n{:?}", Goban::new(enemy, player)); //to remove
        Goban::new(enemy, player)
    }
}

fn launch_ai(input: Goban, player_captures: u8, opponent_captures: u8) -> (u32, u32) {
    let mut algorithm = Algorithm::new();
    algorithm.update_initial_state(input, BitBoard::empty(), player_captures, opponent_captures);
    let ret = algorithm.get_next_move(DEPTH).unwrap();

    get_move_coord(&ret)
}

fn get_move_coord(node: &Node) -> (u32, u32) {
    let move_to_play = node.get_last_move();

    let i: u32 = *move_to_play.get_bit_indexes().last().unwrap() as u32;
    (i / 20, i % 20)
}