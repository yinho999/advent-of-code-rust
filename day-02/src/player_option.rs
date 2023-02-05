use std::str::FromStr;

use thiserror::Error;

pub enum PlayerOption {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for PlayerOption {
    type Err = PlayerOptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(PlayerOption::Rock),
            "B" => Ok(PlayerOption::Paper),
            "C" => Ok(PlayerOption::Scissors),
            "X" => Ok(PlayerOption::Rock),
            "Y" => Ok(PlayerOption::Paper),
            "Z" => Ok(PlayerOption::Scissors),
            _ => Err(PlayerOptionError::InvalidInput(s.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum PlayerOptionError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl PlayerOption {
    pub fn as_usize(&self) -> usize {
        match self {
            PlayerOption::Rock => 1,
            PlayerOption::Paper => 2,
            PlayerOption::Scissors => 3,
        }
    }
}
