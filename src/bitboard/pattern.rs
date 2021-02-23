#[cfg(test)]
mod tests;

use crate::goban::Goban;

use super::{BitBoard, direction::Direction};

pub struct Pattern {
    at_start_line: BitBoard,
    current: BitBoard,
    // A BitBoard that contains the bits which MUST not be set
    // in order to actually match the pattern.
    // Must have the same origin as the pattern itself.
    sub_patterns: Option<Vec<BitBoard>>,
    bits_in_pattern: u16,
    current_x: u8,
    current_y: u8,
    found: bool
}

impl Pattern {
    pub fn from_str(from: &str, sub_patterns: Option<Vec<&str>>) -> Self {
        let main_pattern = BitBoard::from_str(from);
        let sub_patterns = sub_patterns.and_then(|s| Some(s.iter().map(|&p| BitBoard::from_str(p)).collect()));

        Self::new(main_pattern, sub_patterns)
    }

    /// The param `main_pattern` is the pattern used to search in the player's BitBoard.
    /// The param `sub_patterns` is an optional list of sub pattern to match at the same position
    /// as where the main_pattern has matched but on the enemy's BitBoard.
    pub fn new(main_pattern: BitBoard, sub_patterns: Option<Vec<BitBoard>>) -> Self {
        Pattern {
            at_start_line: main_pattern,
            current: main_pattern,
            sub_patterns,
            bits_in_pattern: main_pattern.count_ones(),
            current_x: 0,
            current_y: 0,
            found: false
        }
    }

    /// during the pattern search, thus the pattern should **NOT** be reused.
    // pub fn search_in(&mut self, board: &BitBoard) -> bool {
    //     loop {
    //         if board & &self.current == self.current {
    //             if self.sub_patterns.is_none() {
    //                 self.found = true;
    //                 return true;
    //             }
    //             if let Some(empty_pattern) = self.sub_patterns {
    //                 if (board & &((empty_pattern >> (BitBoard::MOVE_UP_DOWN_SHIFT_VALUE * self.current_y as u32)) >> self.current_x as u32)).is_empty() {
    //                     self.found = true;
    //                     return true;
    //                 }
    //             }
    //         }
    //         if !self.try_move_by_one() {
    //             return false;
    //         }
    //     }
    // }

    /// This method returns an `Option<Vec<bool>>` which contains the result,
    /// in the same order as in the sub pattern list when it was provided,
    /// of the match of every sub pattern on the BitBoard `enemy`.
    ///
    /// If there is no sub patterns provided then the `Vec` will be empty.
    ///
    /// If the main pattern doesn't match anywhere in the player's BitBoard
    /// then the return value is `None`.
    ///
    /// **IMPORTANT**: This method modifies the `main_pattern`'s underlying BitBoard.
    pub fn search_in_goban(&mut self, goban: &Goban) -> Option<Vec<bool>> {
        let player = goban.get_player();
        let enemy = goban.get_enemy();

        if self.found && !self.try_move_by_one() {
            return None;
        }
        self.found = false;

        let mut result = vec![];
        loop {
            if player & &self.current == self.current {
                self.found = true;
                if let Some(sub_patterns) = &self.sub_patterns {
                    for sub_pattern in sub_patterns {
                        result.push((enemy & &((sub_pattern >> (BitBoard::MOVE_UP_DOWN_SHIFT_VALUE * self.current_y as u32)) >> self.current_x as u32)).is_any());
                    }
                }
                break Some(result);
            }
            if !self.try_move_by_one() {
                self.found = false;
                break None;
            }
        }
    }

    pub fn get_pattern_coord(&self) -> (u8, u8) {
        (self.current_x, self.current_y)
    }

    pub fn is_match(&self) -> bool {
        self.found
    }

    /// Try to move the pattern in the BitBoard by one bit.
    /// If this move would make the pattern to goes out of the board,
    /// then the pattern is moved back to the left-most position and moved
    /// down by one line.
    /// If the pattern can't move anymore, this method returns `false`.
    /// It returns `true` in any other cases.
    fn try_move_by_one(&mut self) -> bool {
        let moved = self.current >> Direction::E;

        if moved.count_ones() == self.bits_in_pattern {
            self.current = moved;
            self.current_x += 1;
            return true;
        }

        let moved_down = self.at_start_line >> Direction::S;
        if moved_down.count_ones() == self.bits_in_pattern {
            self.at_start_line = moved_down;
            self.current = moved_down;
            self.current_x = 0;
            self.current_y += 1;
            return true;
        }

        false
    }
}