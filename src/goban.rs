use std::fmt;
use std::fmt::Formatter;

const EMPTY: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;
const BWIDTH: u8 = 19;
const BHEIGHT: u8 = 19;
const BSIZE: u16 = 361;

#[derive(Clone, Debug)]
pub struct Goban
{
    grid: Vec<u8>,
    p_color:u8,
}

impl fmt::Display for Goban {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ret: String = String::with_capacity(400);
        ret.push('\n');
        for (i, x) in self.grid.iter().enumerate()
        {
            if i % 19 == 0 {
                ret.push('\n');
            }
            ret.push_str(&*format!("{} ", x));
        }
        write!(f, "{}", ret)
    }
}

impl std::ops::Index<(usize, usize)> for Goban {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[BWIDTH as usize * y + x]
    }
}

impl Goban
{
    pub fn new(grid: Vec<u8>, p_color: u8) -> Self
    {
        Self
        {
            grid,
            p_color
        }
    }

    pub fn get_move(&self, pos: (usize, usize), dir: Move) -> u8
    {
        match dir {
            Move::Up => self[(pos.0, pos.1 - 1)],
            Move::UpLeft => self[(pos.0 - 1, pos.1 - 1)],
            Move::UpRight => self[(pos.0 + 1, pos.1 - 1)],
            Move::Left => self[(pos.0 - 1, pos.1)],
            Move::Right => self[(pos.0 + 1, pos.1)],
            Move::DownLeft => self[(pos.0 - 1, pos.1 + 1)],
            Move::DownRight => self[(pos.0 + 1, pos.1 + 1)],
            Move::Down => self[(pos.0, pos.1 + 1)],
        }
    }

    pub fn list_pawns(&self) -> Vec<(u8, u8)>
    {
        self.grid.iter().enumerate().filter(|(_, i)| **i == self.p_color).map(|(pc, _)| ((pc % 19) as u8, (pc / 19) as u8)).collect()
    }

    pub fn heuristics(self) -> u32
    {
        self.grid.iter().filter(|i| **i == self.p_color).fold(0, |acc, x| acc + x);

        // TODO Iterator or for loop like check surroundings and deduce direction then check if several aligned to optimize the amount of reads.
        // TODO Node shit
        unimplemented!()
    }
}

pub enum Move
{
    Up,
    UpLeft,
    UpRight,
    Left,
    Right,
    DownLeft,
    DownRight,
    Down
}

#[cfg(test)]
mod tests {
    use crate::goban::{Goban, BLACK, WHITE};

    #[test]
    fn count_pawns() {
        let mut input: Vec<u8> = vec![0; 361];

        input[4] = WHITE;
        input[6] = WHITE;
        input[0] = BLACK;
        input[1] = BLACK;
        input[2] = BLACK;
        input[3] = BLACK;
        input[18] = BLACK;
        input[20] = BLACK;
        input[21] = BLACK;
        input[38] = BLACK;
        input[180] = BLACK;
        let go: Goban = Goban::new(input, BLACK);
        assert_eq!(go.list_pawns(), vec![(0,0), (1, 0), (2, 0), (3, 0), (18, 0), (1, 1), (2, 1), (0, 2), (9, 9)]);
    }
}
