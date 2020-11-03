use std::fmt::Display;

#[derive(Debug)]
pub enum Stone {
    None,
    Black,
    White
}

impl Default for Stone {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let stone = match self {
            Self::None => "\x1B[48;2;105;105;105m \x1B[0m",
            Self::Black => "\x1B[30;48;2;105;105;105m•\x1B[0m",
            Self::White => "\x1B[37;48;2;105;105;105m•\x1B[0m",
        };
        write!(f, "{}", stone)
    }
}

#[cfg(test)]
mod tests {
    use crate::stone::Stone;

    #[test]
    fn test_text_color_for_stone() {
        println!("{}{}{}{}{}\n{}{}{}{}{}\n{}{}{}{}{}\n{}{}{}{}{}\n{}{}{}{}{}\n",
            Stone::Black, Stone::None, Stone::None, Stone::None, Stone::None,
            Stone::White, Stone::Black, Stone::White, Stone::None, Stone::White,
            Stone::None, Stone::White, Stone::Black, Stone::None, Stone::None,
            Stone::None, Stone::Black, Stone::None, Stone::Black, Stone::White,
            Stone::Black, Stone::White, Stone::Black, Stone::None, Stone::Black);
    }
}