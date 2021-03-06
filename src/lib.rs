use numpy::PyArray2;
use pyo3::exceptions;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::types::PyBool;

use goban::Goban;
use node::Node;

use crate::algorithm::Algorithm;
use crate::bitboard::BitBoard;

mod algorithm;
mod bitboard;
mod goban;
mod node;

const DEPTH: u32 = 5;
const WHITE: u8 = 2;

#[pymodule]
fn rust_ext(_py: Python<'_>, _m: &PyModule) -> PyResult<()> {
    #[pyfn(_m, "get_next_move")]
    /// Interfacing function.
    /// Takes the Python GIL, the board in the shape of a 19*19 numpy 2d array, the color of the human player, a boolean that indicates if this is a hint request, and the number of captures made by the human and the ai.
    #[allow(unused_variables)]
    fn get_next_move(_py: Python<'_>, goban: &PyArray2<u8>, p_color: u8, hint: &PyBool, human_capture: i32, ai_capture: i32, last_move_human : Option<(u16, u16)>, last_move_ai : Option<(u16, u16)>) -> PyResult<(u32, u32)> {
        let board:Vec<u8> = goban.to_vec()?;

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
        let last_move = last_move_human.or(last_move_ai);
        let ret = launch_ai(goban, (ai_capture / 2) as u8, (human_capture / 2) as u8, last_move);
        Ok(ret)
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
        Goban::new(enemy, player)
    } else {
        Goban::new(player, enemy)
    }
}

fn launch_ai(input: Goban, player_captures: u8, opponent_captures: u8, last_move: Option<(u16, u16)>) -> (u32, u32) {
    let mut algorithm = Algorithm::new();
    let last_move = if let Some(coord) = last_move {
        BitBoard::one_bit_from_coord(coord)
    } else {
        BitBoard::empty()
    };
    algorithm.update_initial_state(input, last_move, player_captures, opponent_captures);
    let ret = algorithm.get_next_move(DEPTH);

    get_move_coord(&ret)
}

fn get_move_coord(node: &Node) -> (u32, u32) {
    let move_to_play = node.get_last_move();

    let i: u32 = *move_to_play.get_bit_indexes().last().unwrap_or(&0) as u32;
    (i / 20, i % 20)
}