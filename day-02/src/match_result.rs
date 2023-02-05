use std::str::FromStr;

use thiserror::Error;

pub enum MatchResult {
    Win,
    Lose,
    Draw,
}

impl MatchResult {
    pub fn from_result(result: &MatchResult) -> usize {
        match result {
            MatchResult::Win => 6,
            MatchResult::Lose => 0,
            MatchResult::Draw => 3,
        }
    }
}

impl FromStr for MatchResult {
    type Err = MatchResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchResult::Lose),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(MatchResultError::InvalidInput(s.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum MatchResultError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
