use crate::bitboard::axis::AxisIterator;
use crate::bitboard::BitBoard;
use crate::bitboard::direction::Direction;
use crate::goban::fscore::Fscore;
use crate::goban::Goban;
use crate::bitboard::pattern::match_pattern;

///const pattern for heuristic estimation
const PATTERNS_ESTIMATION: [((u8, u8, bool), isize); 3] = [
	((0b11111000, 5, true), 1000isize), //five
	((0b01111000, 6, true), 500isize), //open four
	((0b01110000, 5, true), 250), //open three
];

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

	///Fscore estimation function, suggest to be moved to another file
	pub fn heuristic_estimation(player: BitBoard, enemy : BitBoard) -> Fscore {
		let mut estimation = 0u16;

		//adjust estimation score in regard to the matched pattern
		for &((pattern, pattern_size, is_sym), player_score) in PATTERNS_ESTIMATION.into_iter() {
			let matched = match_pattern(player, enemy, pattern, pattern_size, is_sym);
			estimation += matched.count_ones();
			println!("estimation during match {:?}", estimation);
		}
		//*4 may change don't bother with it too much
		estimation += (player.count_ones() * 4) - (enemy.count_ones() * 4);
		println!("estimation after count ones : {:?}", estimation);

		Fscore::Value(estimation as isize)

		//check patterns les plus importants
		// if 5 score += Fscore(Win)
		// if open 4 score += 10000
		// if close 4 score += 500
		// if open 3 score += 250

		//score += ((layer.count_ones * 4) - (enemy.count_ones * 4)) *4 may change in the future
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
					_ => 0
				};
			}
		}
		Fscore::Value(total)
	}

	// TODO Reimplement neighbour layering somehow? -> I think this won't be necessary
	pub fn compute_heuristic(&self, to_play: &BitBoard) -> Fscore {
		match self.line_detection() {
			Fscore::Win => Fscore::Win,
			Fscore::Value(x) => {
				let neighbour_layering = self.neighbour_layering(to_play) * 10;
				Fscore::Value(neighbour_layering - x) },
			uninit => uninit
		}
	}

	pub fn compute_fscore(
		&mut self,
		previous_state: &Goban,
		to_play: &BitBoard,
		depth: usize) -> Fscore {
		self.fscore = match previous_state.compute_heuristic(to_play) {
			Fscore::Win => Fscore::Win,
			Fscore::Value(x) => Fscore::Value(x + depth as isize),
			uninit => uninit
		};
		self.fscore
	}
}
