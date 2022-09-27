//! A 2-player tic-tac-toe game

use std::fmt::Display;

use anyhow::{anyhow, Ok, Result};

/// Represents the player, but also any square they have played
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

/// Represents the game board itself
pub struct Game {
    next_player: Player,
    arr_squares: [[Option<Player>; 3]; 3],
}

impl Player {
    fn next(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " ___________")?;
        writeln!(f, "|   |   |   |")?;
        for i in 1..=9 {
            let (y, x) = get_coords(i).map_err(|_| std::fmt::Error)?;
            if y != 0 && x == 0 {
                writeln!(f, "|___|___|___|")?;
                writeln!(f, "|   |   |   |")?;
            }
            let str = match self.arr_squares[y][x] {
                Some(Player::X) => "X".to_owned(),
                Some(Player::O) => "O".to_owned(),
                None => i.to_string(),
            };
            write!(f, "| {} ", str)?;
            if x == 2 {
                writeln!(f, "|")?;
            }
        }
        write!(f, "|___|___|___|")
    }
}

impl Game {
    /// Creates a new game with an empty board and next player set to [`Player::X`]
    pub fn new() -> Self {
        let row = [None; 3];
        let arr_squares = [row; 3];

        Self {
            next_player: Player::X,
            arr_squares,
        }
    }

    /// The player variant for the current turn
    pub fn get_player(&self) -> Player {
        self.next_player
    }

    /// Fill the desired square with [`Player::X`] or [`Player::O`] depending on the current player, advancing the turn to
    /// the next player.
    ///
    /// # Errors
    ///
    /// If the square number is not between 1 and 9 (inclusive), or corresponds to a taken square, an
    /// error variant is returned. If an error is returned, it must be guaranteed that the turn was
    /// not advanced to the next player.
    pub fn make_move(&mut self, i: usize) -> Result<()> {
        let coords = get_coords(i)?;
        let target = &mut self.arr_squares[coords.0][coords.1];

        match target {
            None => {
                let current_player = target.insert(self.next_player);
                self.next_player = current_player.next();
                Ok(())
            }
            Some(player) => Err(anyhow!(format!(
                "Tile {i} already is already filled by {}",
                player
            ))),
        }
    }

    /// Indicates whether or not the board is full, useful for tie checking
    pub fn is_full(&self) -> bool {
        self.arr_squares
            .iter()
            .flatten()
            .all(|square| square.is_some())
    }

    /// Returns the winner of the current board or [`None`].
    pub fn get_winner(&self) -> Option<Player> {
        for player in [Some(Player::X), Some(Player::O)] {
            for row in 0..3 {
                if self.arr_squares[row].iter().all(|square| square == &player) {
                    return player;
                }
            }

            for col in 0..3 {
                if self
                    .arr_squares
                    .iter()
                    .map(|row| row[col])
                    .all(|square| square == player)
                {
                    return player;
                }
            }

            if (
                self.arr_squares[0][0],
                self.arr_squares[1][1],
                self.arr_squares[2][2],
            ) == (player, player, player)
            {
                return player;
            }

            if (
                self.arr_squares[0][2],
                self.arr_squares[1][1],
                self.arr_squares[2][0],
            ) == (player, player, player)
            {
                return player;
            }
        }

        None
    }
}

fn get_coords(i: usize) -> Result<(usize, usize)> {
    if i < 1 || i > 9 {
        return Err(anyhow!("Input must be between 1 and 9".to_owned()));
    }
    let i = i - 1;

    Ok((i / 3, i % 3))
}

#[cfg(test)]
mod tests {
    use super::*;
    use Player::*;

    #[test]
    fn next_player() {
        let x = X;
        let o = x.next();
        assert_eq!(O, o);
        assert_eq!(X, o.next());
    }

    #[test]
    fn display_player() {
        assert_eq!("X".to_owned(), X.to_string());
        assert_eq!("O".to_owned(), O.to_string());
    }

    #[test]
    fn display_game() -> Result<()> {
        let mut game = Game::new();
        assert_eq!(
            " ___________
|   |   |   |
| 1 | 2 | 3 |
|___|___|___|
|   |   |   |
| 4 | 5 | 6 |
|___|___|___|
|   |   |   |
| 7 | 8 | 9 |
|___|___|___|"
                .to_owned(),
            game.to_string()
        );
        game.make_move(1)?;
        assert_eq!(
            " ___________
|   |   |   |
| X | 2 | 3 |
|___|___|___|
|   |   |   |
| 4 | 5 | 6 |
|___|___|___|
|   |   |   |
| 7 | 8 | 9 |
|___|___|___|"
                .to_owned(),
            game.to_string()
        );
        Ok(())
    }

    #[test]
    fn new_game() {
        let game = Game::new();
        for i in game.arr_squares.into_iter().flatten() {
            assert_eq!(None, i);
        }
        assert_eq!(game.next_player, Player::X);
    }

    #[test]
    fn get_player_test() -> Result<()> {
        let mut game = Game::new();
        assert_eq!(X, game.get_player());
        game.make_move(5)?;
        assert_eq!(O, game.get_player());
        Ok(())
    }

    #[test]
    fn make_valid_move() -> Result<()> {
        let mut game = Game::new();
        game.make_move(5)?;
        assert_eq!(Some(X), game.arr_squares[1][1]);
        Ok(())
    }

    #[test]
    fn make_out_of_range_move() {
        let mut game = Game::new();
        assert!(game.make_move(99).is_err());
    }

    #[test]
    fn make_invalid_move() -> Result<()> {
        let mut game = Game::new();
        game.make_move(5)?;
        assert!(game.make_move(5).is_err());
        Ok(())
    }

    #[test]
    fn is_full() {
        let mut game = Game::new();
        game.arr_squares = [
            [Some(X), Some(O), Some(X)],
            [Some(O), Some(O), Some(X)],
            [Some(O), Some(X), Some(X)],
        ];
        assert!(game.is_full());
    }

    #[test]
    fn is_not_full() {
        let mut game = Game::new();
        game.arr_squares = [
            [Some(X), Some(O), Some(X)],
            [Some(O), None, Some(X)],
            [Some(O), Some(X), Some(X)],
        ];
        assert!(!game.is_full());
    }

    #[test]
    fn draw_checking() {
        let mut game = Game::new();
        game.arr_squares = [
            [Some(X), Some(O), Some(O)],
            [Some(O), Some(X), Some(X)],
            [Some(O), Some(X), Some(O)],
        ];
        assert_eq!(None, game.get_winner());
    }

    #[test]
    fn loser_checking() {
        let mut game = Game::new();
        game.arr_squares = [
            [Some(X), Some(X), Some(O)],
            [None, Some(O), Some(X)],
            [Some(O), Some(O), Some(X)],
        ];
        assert_eq!(Some(O), game.get_winner());
    }

    #[test]
    fn winner_checking() {
        let mut game = Game::new();
        game.arr_squares = [
            [Some(X), Some(O), Some(X)],
            [Some(O), Some(O), Some(X)],
            [Some(O), Some(X), Some(X)],
        ];
        assert_eq!(Some(X), game.get_winner());
    }

    #[test]
    fn coords_test() -> Result<()> {
        let tests = [
            (1, 0, 0),
            (2, 0, 1),
            (3, 0, 2),
            (4, 1, 0),
            (5, 1, 1),
            (6, 1, 2),
            (7, 2, 0),
            (8, 2, 1),
            (9, 2, 2),
        ];
        for t in tests {
            assert_eq!((t.1, t.2), get_coords(t.0)?);
        }
        Ok(())
    }
}
