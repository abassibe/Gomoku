use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::ops::{BitAnd, BitOr, BitXor};

use fscore::Fscore;

use super::bitboard::*;

#[cfg(test)]
mod tests;
pub mod heuristic;
pub mod fscore;

#[derive(Clone, Debug, Default, Copy)]
pub struct Goban {
	board: BitBoard,
	fscore: Fscore,
	computer: BitBoard,
	human: BitBoard
}


impl Goban {
	pub fn new(computer: BitBoard, human: BitBoard) -> Self {
		Self {
			fscore: Fscore::Uninitialized,
			computer,
			human,
			board: computer | human
		}
	}

	pub fn get_computer(&self) -> &BitBoard {
		&self.computer
	}

	pub fn get_human(&self) -> &BitBoard {
		&self.human
	}

	pub fn get_board(&self) -> BitBoard {
		self.board
	}

	pub fn get_fscore(&self) -> Fscore {
		self.fscore
	}

	pub fn set_fscore(&mut self, fscore: Fscore) {
		self.fscore = fscore;
	}
}


impl BitAnd<BitBoard> for Goban {
	type Output = BitBoard;

	fn bitand(self, rhs: BitBoard) -> Self::Output {
		self.board & rhs
	}
}

impl BitXor<BitBoard> for Goban {
	type Output = BitBoard;

	fn bitxor(self, rhs: BitBoard) -> Self::Output {
		self.board ^ rhs
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

impl fmt::Display for Goban {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.computer | self.human)
	}
}

impl Hash for Goban {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.fscore.hash(state)
	}
}
