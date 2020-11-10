use super::bitboard::*;
use crate::bitboard::axis::AxisIterator;
use crate::bitboard::direction::Direction;

#[derive(Clone, Debug)]
pub struct Goban
{
    player: BitBoard,
    enemy: BitBoard,
    p_color:u8,
}

impl Goban
{
    pub fn new(player: BitBoard, enemy: BitBoard, p_color: u8) -> Self
    {
        Self
        {
            player,
            enemy,
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
        ((self.enemy | self.player) + (Direction::All)) & BitBoard::empty()
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
	            println!("{}", bits);
                bits = bits - dir;
                len += 1;
            }
            total = len;
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
    use crate::goban::{Goban};
    use crate::bitboard::BitBoard;

    #[test]
    fn neighbours()
    {
        let original = BitBoard::from_array([
            0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            0b00000000000000000000000000000000000000000000000000000000000000000000000111110000000000000000000000000000000000000000000000000000,
            0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
        ]);
        let board = Goban::new(original,  BitBoard::new(0, 0, 0), 2);
        println!("{}\n------", original);
        println!("{}", board.list_neighbours());
    }
    #[test]
    fn alignment()
    {
        let original = BitBoard::from_array([
            0b11111000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
        ]);
        let board = Goban::new(original,  BitBoard::new(0, 0, 0), 2);

        assert_eq!(5000, board.line_detection());

        // let stre: String = String::from("\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n\
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0\n");


    }
}
