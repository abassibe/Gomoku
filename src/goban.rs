use crate::{bitboard::axis::AxisIterator, algorithm};
use crate::bitboard::direction::Direction;

use super::bitboard::*;
use std::fmt::{Formatter};
use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{BitOr, BitAnd, BitXor};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Fscore {
	Uninitialized,
	Value(isize),
	Win
}

impl Fscore {
	pub fn is_win(&self) -> bool {
		*self == Fscore::Win
	}

	pub fn is_initialized(&self) -> bool {
		!(*self == Fscore::Uninitialized)
	}
}

impl Default for Fscore {
	fn default() -> Self { Fscore::Uninitialized }
}

impl Ord for Fscore {
	fn cmp(&self, other: &Self) -> Ordering {
		let self_has_value = if let Fscore::Value(_) = self { true } else { false };
		let other_has_value = if let Fscore::Value(_) = other { true } else { false };
		if !self_has_value || !other_has_value {
			let self_as_u8: u8 = self.into();
			return self_as_u8.cmp(&other.into());
		}
		// At this point both `self` and `other` should be of type `Fscore::Value`
		let self_value = if let Fscore::Value(x) = self { *x } else { isize::MIN };
		let other_value = if let Fscore::Value(x) = other { *x } else { isize::MIN };
		self_value.cmp(&other_value)
	}
}

impl PartialOrd for Fscore {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Into<u8> for Fscore {
	fn into(self) -> u8 {
		match self {
			Fscore::Uninitialized => 0,
			Fscore::Value(_) => 1,
			Fscore::Win => 2
		}
	}
}

impl Into<u8> for &Fscore {
	fn into(self) -> u8 {
		match self {
			Fscore::Uninitialized => 0,
			Fscore::Value(_) => 1,
			Fscore::Win => 2
		}
	}
}

impl fmt::Display for Fscore {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Fscore::Uninitialized => "Uninitialized".into(),
			Fscore::Value(x) => x.to_string(),
			Fscore::Win => "Win".into()
		})
	}
}

#[derive(Clone, Debug, Default, Copy)]
pub struct Goban
{
	fscore: Fscore,
	board: BitBoard,
	player: BitBoard,
	enemy: BitBoard,
}

impl Goban
{
	pub fn new(player: BitBoard, enemy: BitBoard) -> Self
	{
		Self
		{
			fscore: Fscore::Value(0),
			player,
			enemy,
			board: player | enemy,
        }
	}

	pub fn get_fscore(&self) -> Fscore {
		self.fscore
	}

	pub fn set_fscore(&mut self, fscore: Fscore) {
		self.fscore = fscore;
	}

	pub fn list_moves(&self) -> BitBoard
	{
		!self.board
	}

	// TODO: Forbidden moves
	pub fn check_illegal(&self) -> bool
	{
		todo!()
	}

	pub fn neighbour_layering(&self, to_play: &BitBoard) -> isize
	{
		let mut layers = self.board;
		let mut ret: isize = 0;
		while (&layers & to_play).is_empty()
		{
			layers |= layers + Direction::All;
			ret += 1;
		}
		ret
	}

	pub fn list_neighbours(&self) -> BitBoard {
		(self.board + Direction::All) & self.list_moves()
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

	pub fn get_player(&self) -> &BitBoard {
		&self.player
	}

	pub fn get_enemy(&self) -> &BitBoard {
		&self.enemy
	}

	// TODO Add way to isolate different lines, currently cannot differentiate between lines that are on the same axes
	fn line_detection(&self) -> Fscore
	{
		let mut bits: BitBoard;
		let mut final_line: BitBoard;
		let mut total: isize = 0;
		let mut len: isize;

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
				else if len == 5 {
					return Fscore::Win
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
				// println!("Change Direction (Current: {:?})\nTotal = {}\n", dir, total);
				// println!("^-------------------------^\n{}v-------------------------v", final_line);
			}
		}
		Fscore::Value(total)
	}

	// TODO Reimplement neighbour layering somehow? -> I think this won't be necessary
	pub fn compute_heuristic(&self, to_play: &BitBoard) -> Fscore
	{
		// (self.neighbour_layering(to_play) - self.line_detection()) as u64
		match self.line_detection() {
			Fscore::Win => Fscore::Win,
			Fscore::Value(x) => {
				println!("to_play:\n{}", to_play);
				println!("self.board:\n{}", self.board);
				let neighbour_layering = self.neighbour_layering(to_play) * 10;
				Fscore::Value(neighbour_layering - x)
			},
			uninit => uninit
		}
	}

	pub fn compute_fscore(&mut self, previous_state: &Goban, to_play: &BitBoard, depth: usize) -> Fscore
	{
		self.fscore = match previous_state.compute_heuristic(to_play) {
			Fscore::Win => Fscore::Win,
			Fscore::Value(x) => Fscore::Value(x + depth as isize),
			uninit => uninit
		};
		self.fscore
	}
}

impl BitAnd<BitBoard> for Goban {
	type Output = BitBoard;

	fn bitand(self, rhs: BitBoard) -> Self::Output {
		self.board | rhs
	}
}

impl BitXor<BitBoard> for Goban {
	type Output = BitBoard;

	fn bitxor(self, rhs: BitBoard) -> Self::Output {
		self.board | rhs
	}
}

impl BitOr<BitBoard> for Goban {
	type Output = BitBoard;

	fn bitor(self, rhs: BitBoard) -> Self::Output {
		self.board | rhs
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

    use super::Fscore;

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
		assert_eq!(4, board.neighbour_layering(&to_play));
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

		assert_eq!(Fscore::Value(7), board.line_detection());

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
