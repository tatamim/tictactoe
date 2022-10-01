use std::fmt::Display;

use colored::{ColoredString, Colorize};

/// Represents the player, but also any square they have played
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn next(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    pub fn colored(&self) -> ColoredString {
        match self {
            Self::X => "X".blue(),
            Self::O => "O".red(),
        }
    }

    pub fn colored_highlighted(&self) -> ColoredString {
        self.colored().bold().underline()
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.colored())
    }
}

#[cfg(test)]
mod tests {
    use crate::Player::*;
    use colored::Colorize;

    #[test]
    fn next_player() {
        let x = X;
        let o = x.next();
        assert_eq!(O, o);
        assert_eq!(X, o.next());
    }

    #[test]
    fn player_is_colored() {
        assert_eq!(X.colored(), "X".blue());
        assert_eq!(O.colored(), "O".red());
    }

    #[test]
    fn player_is_colored_bold_underlined() {
        assert_eq!(X.colored_highlighted(), "X".blue().bold().underline());
        assert_eq!(O.colored_highlighted(), "O".red().bold().underline());
    }
}
