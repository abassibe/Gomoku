use std::fmt;
use std::fmt::Formatter;
use super::bitboard::*;
use crate::bitboard::axis::AxisIterator;

const EMPTY: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;
const BWIDTH: u8 = 19;
const BHEIGHT: u8 = 19;
const BSIZE: u16 = 361;

#[derive(Clone, Debug)]
pub struct Goban
{
    player: BitBoard,
    enemy: BitBoard,
    p_color:u8,
}

impl Goban
{
    pub fn new(white: BitBoard, black: BitBoard, p_color: u8) -> Self
    {
        Self
        {
            player: white,
            enemy: black,
            p_color
        }
    }

    pub fn list_moves(&self) -> BitBoard
    {
        !(self.enemy | self.player)
    }

    // TODO: Neighbour layering
    pub fn list_neighbours(&self) -> BitBoard
    {
        (self.enemy | self.player).dilate(Direction::All) & BitBoard::empty()
    }

    fn line_detection(&self) -> u16
    {
        let mut bits: BitBoard;
        let mut total: u16 = 0;
        let mut len: u16;

        for dir in AxisIterator::new()
        {
	        bits = self.player;
            len = 0;
            while !bits.is_empty()
            {
                bits = bits.erode(dir);
                len += 1;
                if len == 5 {
                    len = 5000;
                    break;
                }
            }
            total += len;
        }
	    total
    }

    pub fn get_heuristic(&self) -> i64
    {
        let mut ret: i64 = 0;

        ret += self.line_detection() as i64;
	    ret
    }
}

#[cfg(test)]
mod tests {
    use crate::goban::{Goban, BLACK, WHITE};
}
