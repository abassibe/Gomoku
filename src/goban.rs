use crate::bitboard::axis::AxisIterator;
use crate::bitboard::direction::Direction;

use super::bitboard::*;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};


#[derive(Clone, Debug)]
pub struct Goban
{
	fscore: usize,
	player: BitBoard,
	enemy: BitBoard,
}

impl Goban
{
	pub fn new(player: BitBoard, enemy: BitBoard) -> Self
	{
		Self
		{
			fscore: 0,
			player,
			enemy,
        }
	}

	pub fn list_moves(&self) -> BitBoard
	{
		!(self.enemy | self.player)
	}

	// TODO: Forbidden moves
	pub fn check_illegal(&self) -> bool
	{
		todo!()
	}

	// TODO: Neighbour layering
	pub fn neighbour_layering(&self, to_play: BitBoard)
	{
		todo!()
	}

	pub fn list_neighbours(&self) -> BitBoard {
		((self.enemy | self.player) + Direction::All) & self.list_moves()
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

impl Eq for Goban {}

impl PartialEq for Goban {
	fn eq(&self, other: &Self) -> bool {
		self.fscore == other.fscore
	}
}

impl Ord for Goban {
	fn cmp(&self, other: &Self) -> Ordering {
		self.fscore.cmp(&other.fscore)
	}
}

impl PartialOrd for Goban {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl fmt::Display for Goban
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.player | self.enemy)
	}
}

impl Hash for Goban {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.fscore.hash(state)
	}
}

	#[cfg(test)]
mod tests {
	use crate::bitboard::BitBoard;
	use crate::goban::Goban;

	#[test]
	fn neighbours()
	{
		let player = BitBoard::from_array([
			0b00000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000011111000000000000001111100000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
		]);
		let enemy = BitBoard::from_array([
			0b00000000000000000000000000000000000000000000000000000000000000000000001100000000000000000110000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000011100000000000001110000000000000000000000
		]);
		let expected = BitBoard::from_array([
			0b00000000000000000000000000000000000000000000000000111100000000000000010010000000000000001001000000000000000111100000000000000000,
			0b00000000000000000000000000000000000000000000001111111000000000000100000100000000000010000010000000000001111111000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000001111100000000000111100010000000000010001111000000000001111100
		]);
		let board = Goban::new(player, enemy);
		println!("PLAYER\n{}\nENEMY\n{}\nFULL\n{}", player, enemy, player | enemy);
		println!("RESULT\n{}\nEXPECTED\n{}", board.list_neighbours(), expected);
		assert_eq!(board.list_neighbours(), expected);
	}

	#[test]
	fn alignment()
	{
		let original = BitBoard::from_array([
			0b11111000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
		]);
		let board = Goban::new(original, BitBoard::new(0, 0, 0));

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
