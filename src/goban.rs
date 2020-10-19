use std::fmt;
use crate::{BWIDTH};
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Goban
{
    grid: Vec<u8>,
    p_color:u8,
}

impl fmt::Display for Goban {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ret: String = String::with_capacity(500);
        for (i, x) in self.grid.iter().enumerate()
        {
            ret.push_str(&*format!("{} ", x));
            if i % 19 == 0 {
                ret.push('\n');
            }
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
    #[test]
    fn display() {
        assert_eq!(2 + 2, 4);
    }
}
