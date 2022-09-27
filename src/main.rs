mod game;

use std::{
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

use anyhow::{anyhow, Result};

use game::{Game, Player};

fn main() -> Result<()> {
    let mut game = Game::new();
    let winner: Option<Player> = loop {
        println!("{}", game);
        loop {
            print!("It's {}'s turn: ", game.get_player());
            stdout().flush()?;
            let mut str = String::new();
            if stdin().read_line(&mut str).is_err() {
                println!("Could not read the line");
                continue;
            }
            match str
                .trim()
                .parse::<usize>()
                .map_err(|err: ParseIntError| anyhow!(err))
                .and_then(|i| game.make_move(i))
            {
                Ok(_) => break,
                Err(err) => println!("{err}"),
            };
        }
        if let Some(player) = game.get_winner() {
            break Some(player);
        } else if game.is_full() {
            break None;
        }
    };
    println!("{}", game);
    match winner {
        Some(winner) => println!("The winner is {}", winner),
        None => println!("It's a draw"),
    }
    Ok(())
}
