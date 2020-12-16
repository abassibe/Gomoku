use crate::bitboard::axis::AxisIterator;
use crate::bitboard::direction::Direction;

use super::bitboard::*;
use std::fmt::{Formatter};
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

	pub fn neighbour_layering(&self, to_play: BitBoard) -> u16
	{
		let mut layers = self.player | self.enemy;
		let mut ret: u16 = 0;
		while (layers & to_play).is_empty()
		{
			layers |= layers + Direction::All;
			ret += 1;
		}
		ret
	}

	pub fn list_neighbours(&self) -> BitBoard {
		((self.enemy | self.player) + Direction::All) & self.list_moves()
	}

	fn check_surround(&self, lines: BitBoard, dir: Direction) -> u8
	{
		let mut ret: u8 = 0;
		for dirs in [dir, dir.to_invert()].iter()
		{
			if ((lines >> *dirs) & self.enemy).is_empty() {
				ret += 1;
			}
		}
		ret
	}

	// TODO Add way to isolate different lines, currently cannot differentiate between lines that are on the same axes
	fn line_detection(&self) -> u16
	{
		let mut bits: BitBoard;
		let mut final_line: BitBoard;
		let mut total: u16 = 0;
		let mut len: u16;

		for dir in AxisIterator::new()
		{
			bits = self.player;
			final_line = BitBoard::empty();
			len = 0;
			while bits.is_any()
			{
				if len == 1 {
					final_line = bits + dir.to_invert();
				}
				bits = bits - dir;
				len += 1;
			}
			if len > 1 {
				total += match self.check_surround(final_line, dir)
				{
					2 => len,
					1 => len / 2,
					_ => 0,
				};
				println!("Change Direction (Current: {:?})\nTotal = {}\n", dir, total);
				println!("^-------------------------^\n{}v-------------------------v", final_line);
			}
		}
		total
	}

	pub fn get_heuristic(&self, to_play: BitBoard) -> u64
	{
		(self.neighbour_layering(to_play) - self.line_detection()) as u64
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
	fn neighbour_layers()
	{
		let to_play = BitBoard::from_str("
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000010000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		");

		let player = BitBoard::from_str("
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000010000000000
		0000000010000000000
		0000000001000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		");

		let enemy = BitBoard::from_str("
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000010000000
		0000000000100000000
		0000000001000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		");

		let board = Goban::new(player, enemy);

		println!("{}", to_play);
		assert_eq!(4, board.neighbour_layering(to_play));
	}

	#[test]
	fn neighbours()
	{

		// 00000000000000000012
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00010000000000000002
		// 00010000000000000002
		// 00010000000000111112
		// 00010000000001111102
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002

		let player = BitBoard::from_array([
			0b00000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
			0b00000000000000010000000000000000000100000000000000000001000000000011111000010000000001111100000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
		]);

		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000001100000002
		// 00000000011000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000011111000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00011100000000000002
		// 11000000000000000002

		let enemy = BitBoard::from_array([
			0b00000000000000000000000000000000000000000000000000000000000000000000001100000000000000000110000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
			0b00000000001111100000000000000000000000000000000000000000000000000000000000000000000000011100000000000000110000000000000000000000,
		]);
		let expected = BitBoard::from_array([
			0b00000000000000000100000000000000000001100000000001111000000000000000110010000000000000001001100000000000000011110000000000111000,
			0b00000000000000101000000000000000001010000000011111100010100000001100000000101000000010000010001110000000111111100000011111110000,
			0b00000000010000010000000000000111111100000000000000000000000000000011111000000000000011100010000000000000001111100000000000000000,
		]);
		let board = Goban::new(player, enemy);
		println!("PLAYER\n{}\nENEMY\n{}\nFULL\n{}", player, enemy, player | enemy);
		println!("RESULT\n{}\nEXPECTED\n{}", board.list_neighbours(), expected);
		assert_eq!(board.list_neighbours(), expected);
	}

	#[test]
	fn alignment()
	{

		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000100000002
		// 00000000001000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000011111000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00000000000000000002
		// 00011100000000000002
		// 00000000000000000002

		let original = BitBoard::from_array([
			0b10000000000000000000000000000000000000000000000000000000000000000000000100000000000000000010000000000000000000000000000000000000,
			0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
			0b00000000001111100000000000000000000000000000000000000000000000000000000000000000000000011100000000000000000000000000000000000000,
		]);
		let board = Goban::new(original, BitBoard::empty());
		println!("HSCORE= {}", board.line_detection());

		assert_eq!(7, board.line_detection());

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
