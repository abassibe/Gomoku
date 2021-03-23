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

// TODO Put fscore in Node because I was wrong lmao
#[derive(Clone, Debug, Default, Copy)]
pub struct Goban {
	board: BitBoard,
	fscore: Fscore,
	player: BitBoard,
	enemy: BitBoard
}


impl Goban {
	pub fn new(player: BitBoard, enemy: BitBoard) -> Self {
		Self {
			fscore: Fscore::Value(heuristic::set_heuristic_estimation(player, enemy)),
			player,
			enemy,
			board: player | enemy
		}
	}

	pub fn get_player(&self) -> &BitBoard {
		&self.player
	}

	pub fn get_enemy(&self) -> &BitBoard {
		&self.enemy
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
		if self.player == other.player && self.enemy == other.enemy {
			true
		} else {
			false
		}
	}
}

impl fmt::Display for Goban {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.player | self.enemy)
	}
}

impl Hash for Goban {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.player.hash(state);
		self.enemy.hash(state);
	}
}
