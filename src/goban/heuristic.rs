use crate::bitboard::axis::AxisIterator;
use crate::bitboard::BitBoard;
use crate::bitboard::direction::Direction;
use crate::goban::fscore::Fscore;
use crate::goban::Goban;

impl Goban {
	/// This method returns a negative version of `self's` bitboard
	pub fn list_moves(&self) -> BitBoard {
		!self.board
	}

	// TODO: Forbidden moves
	pub fn check_illegal(&self) -> bool {
		todo!()
	}

	/// This method returns the distance (number of layers) between the input and `self`
	pub fn neighbour_layering(&self, to_play: &BitBoard) -> isize {
		let mut layers = self.board;
		let mut ret: isize = 0;
		while (&layers & to_play).is_empty() {
			layers |= layers + Direction::All;
			ret += 1;
		}
		ret
	}

	/// This method returns a bitboard where the only set bits are the ones around the bits of self's bitboard.
	pub fn list_neighbours(&self) -> BitBoard {
		(self.board + Direction::All) & self.list_moves()
	}

	fn check_surround(&self, lines: BitBoard, dir: Direction) -> u8 {
		let mut ret: u8 = 0;
		for dirs in [dir, dir.to_invert()].iter() {
			if ((lines >> *dirs) & self.enemy).is_empty() {
				ret += 1;
			}
		}
		ret
	}

	// TODO Add way to isolate different lines, currently cannot differentiate between lines that are on the same axes
	pub fn line_detection(&self) -> Fscore {
		let mut bits: BitBoard;
		let mut final_line: BitBoard;
		let mut total: isize = 0;
		let mut len: isize;

		for dir in AxisIterator::new() {
			bits = self.player;
			final_line = BitBoard::empty();
			len = 0;
			while bits.is_any() {
				if len == 1 {
					final_line = bits + dir.to_invert();
				} else if len >= 4 {
					return Fscore::Win;
				}
				bits = bits - dir;
				len += 1;
			}
			if len > 1 {
				total += match self.check_surround(final_line, dir) {
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
	pub fn compute_heuristic(&self, to_play: &BitBoard) -> Fscore {
		// (self.neighbour_layering(to_play) - self.line_detection()) as u64
		match self.line_detection() {
			Fscore::Win => Fscore::Win,
			Fscore::Value(x) => {
				let neighbour_layering = self.neighbour_layering(to_play) * 10;
				Fscore::Value(neighbour_layering - x)
			}
			uninit => uninit,
		}
	}

	pub fn compute_fscore(
		&mut self,
		previous_state: &Goban,
		to_play: &BitBoard,
		depth: usize,
	) -> Fscore {
		self.fscore = match previous_state.compute_heuristic(to_play) {
			Fscore::Win => Fscore::Win,
			Fscore::Value(x) => Fscore::Value(x + depth as isize),
			uninit => uninit,
		};
		self.fscore
	}
}
