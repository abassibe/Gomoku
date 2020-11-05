use std::fmt;
use std::fmt::Formatter;
use super::bitboard::*;

const EMPTY: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;
const BWIDTH: u8 = 19;
const BHEIGHT: u8 = 19;
const BSIZE: u16 = 361;

#[derive(Clone, Debug)]
pub struct Goban
{
    white: BitBoard,
    black: BitBoard,
    p_color:u8,
}

impl std::ops::Index<(usize, usize)> for Goban {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[BWIDTH as usize * y + x]
    }
}

impl Goban
{
    pub fn new(white: BitBoard, black: BitBoard, p_color: u8) -> Self
    {
        Self
        {
            white,
            black,
            p_color
        }
    }

    pub fn list_moves(&self) -> BitBoard
    {

        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move
{
    Up,
    UpLeft,
    UpRight,
    Left,
    Right,
    DownLeft,
    DownRight,
    Down
}

#[cfg(test)]
mod tests {
    use crate::goban::{Goban, BLACK, WHITE};
}
