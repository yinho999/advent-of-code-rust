use std::str::FromStr;

use day_02::{
    game::Game,
    load_file,
    match_result::{MatchResult, MatchResultError},
    player_option::{PlayerOption, PlayerOptionError},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum GameError {
    #[error("PlayerOptionError: {0}")]
    PlayerOptionError(#[from] PlayerOptionError),
    #[error("MatchResultError: {0}")]
    MatchResultError(#[from] MatchResultError),
}

fn part_one(file_name: &str) -> Result<usize, GameError> {
    let file = load_file(file_name);
    let mut score = 0;
    for line in file {
        let line = line
            .split(" ")
            .map(|chunk| chunk.to_string())
            .collect::<Vec<String>>();
        let game = Game::new(
            line[0].to_owned(),
            PlayerOption::from_str(line[1].as_str())?,
        )?;
        score += game.play();
    }
    Ok(score)
}

fn part_two(file_name: &str) -> Result<usize, GameError> {
    let file = load_file(file_name);
    let mut score = 0;
    for line in file {
        let line = line
            .split(" ")
            .map(|chunk| chunk.to_string())
            .collect::<Vec<String>>();
        let game = Game::new(line[0].to_owned(), MatchResult::from_str(&line[1])?)?;
        score += game.play();
    }
    Ok(score)
}

fn main() -> Result<(), GameError> {
    let score = part_one("data.txt")?;
    println!("{}", score);
    let score = part_two("data.txt")?;
    println!("{}", score);
    Ok(())
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), GameError> {
        let score = part_one("example.txt")?;
        assert_eq!(score, 15);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), GameError> {
        let score = part_two("example.txt")?;
        assert_eq!(score, 12);
        Ok(())
    }
}
