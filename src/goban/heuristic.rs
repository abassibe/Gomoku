// use crate::bitboard::axis::AxisIterator;
use crate::bitboard::BitBoard;
// use crate::bitboard::direction::Direction;
// use crate::goban::fscore::Fscore;
// use crate::goban::Goban;
use crate::bitboard::pattern::match_pattern;

///const pattern for heuristic estimation
const PATTERNS_ESTIMATION: [((u8, u8, bool), isize); 3] = [
	((0b11111000, 5, true), 1000isize), //five
	((0b01111000, 6, true), 500isize), //open four
	((0b01110000, 5, true), 250), //open three
];

///Fscore estimation function (actually quiesce??), suggested being moved to another file
pub fn set_heuristic_estimation(player: BitBoard, enemy: BitBoard) -> isize {
	let mut estimation = 0u16;

	//adjust estimation score in regard to the matched pattern
	for &((pattern, pattern_size, is_sym), _) in PATTERNS_ESTIMATION.iter() {
		let matched = match_pattern(player, enemy, pattern, pattern_size, is_sym);
		estimation += matched.count_ones();
	}
	//*4 may change don't bother with it too much
	let player_count = player.count_ones() * 4;
	let enemy_count = enemy.count_ones() * 4;

	if player_count < enemy_count {
		estimation += enemy_count - player_count
	} else {
		estimation += player_count - enemy_count
	}
    estimation as isize
}
