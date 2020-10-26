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
            ret.push_str(&*format!("{:?} ", x));
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

    pub fn get_pos(&self, pos: (usize, usize), dir: Move) -> Option<(usize, usize)>
    {
        match dir {
            Move::Up => if pos.1 != 0 { Some((pos.0, pos.1 - 1)) } else { None },
            Move::UpLeft => if pos.1 != 0 && pos.0 != 0 { Some((pos.0 - 1, pos.1 - 1)) } else { None },
            Move::UpRight => if pos.1 != 0 && pos.0 < 19 { Some((pos.0 + 1, pos.1 - 1)) } else { None },
            Move::Left => if pos.0 != 0 { Some((pos.0 - 1, pos.1)) } else { None },
            Move::Right => if pos.0 < 19 { Some((pos.0 + 1, pos.1)) } else { None },
            Move::DownLeft => if pos.0 != 0 && pos.1 < 19 { Some((pos.0 - 1, pos.1 + 1)) } else { None },
            Move::DownRight => if pos.1 < 19 && pos.0 < 19 { Some((pos.0 + 1, pos.1 + 1)) } else { None },
            Move::Down => if pos.1 < 19 { Some((pos.0, pos.1 + 1)) } else { None },
        }
    }

    fn list_pawns(&self) -> Vec<(usize, usize)>
    {
        self.grid.iter().enumerate().filter(|(_, i)| **i == self.p_color).map(|(pc, _)| (pc % 19, pc / 19)).collect()
    }

    fn check_line(&self, pos: (usize, usize), dir: Move) -> u32
    {
        let mut ret : u32 = 0;
        let mut next = self.get_pos(pos, dir);

        println!("Checking...");
        while next.is_some() && self[next.unwrap()] == self.p_color
        {
            next = self.get_pos(next.unwrap(), dir);
            ret *= 10;
            println!("In align ret= {:?}, pos= {:?}", ret, pos);
        }
	    ret
    }

    pub fn heuristics(&self) -> u32
    {
        let mut ret: u32 = 0;
        let pawns: Vec<(usize, usize)> = self.list_pawns();

        for (x, y) in pawns {
            for val in [Move::Down, Move::DownRight, Move::DownLeft, Move::Right,
                                        Move::Left, Move::Up, Move::UpRight, Move::UpLeft].iter()
            {
                println!("Pos: {:?} dir:{:?}", (x, y), val);
                ret += self.check_line((x, y), *val)
            }
        }
        ret
        // TODO Flag pawns that were found in alignment
    }
}

#[derive(Copy, Clone, Debug)]
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

    #[test]
    fn heuristics_diag() {
        let mut input: Vec<u8> = vec![0; 361];

        input[0] = BLACK;
        input[1] = BLACK;
        input[2] = BLACK;
        input[3] = BLACK;
        input[4] = BLACK;
        let go: Goban = Goban::new(input, BLACK);
	    let ret = go.heuristics();
        println!("{}\n{:?}", go, ret);
	    assert_eq!(ret, 10000);
    }
}
