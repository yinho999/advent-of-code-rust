use std::str::FromStr;

use crate::{
    match_result::MatchResult,
    player_option::{PlayerOption, PlayerOptionError},
};

pub struct Game<T> {
    input_1: PlayerOption,
    input_2: T,
}

impl<T> Game<T> {
    pub fn new(input_1: String, input_2: T) -> Result<Self, PlayerOptionError> {
        Ok(Game {
            input_1: PlayerOption::from_str(input_1.as_str())?,
            input_2,
        })
    }
}
impl Game<PlayerOption> {
    pub fn play(&self) -> usize {
        let match_result_score = match self.input_1 {
            PlayerOption::Rock => match &self.input_2 {
                PlayerOption::Rock => 3,
                PlayerOption::Paper => 0,
                PlayerOption::Scissors => 6,
            },
            PlayerOption::Paper => match &self.input_2 {
                PlayerOption::Rock => 6,
                PlayerOption::Paper => 3,
                PlayerOption::Scissors => 0,
            },
            PlayerOption::Scissors => match &self.input_2 {
                PlayerOption::Rock => 0,
                PlayerOption::Paper => 6,
                PlayerOption::Scissors => 3,
            },
        };
        match_result_score + self.input_2.as_usize()
    }
}

impl Game<MatchResult> {
    pub fn play(&self) -> usize {
        let player_choice = match self.input_2 {
            MatchResult::Win => match self.input_1 {
                PlayerOption::Rock => PlayerOption::Paper,
                PlayerOption::Paper => PlayerOption::Scissors,
                PlayerOption::Scissors => PlayerOption::Rock,
            },
            MatchResult::Lose => match self.input_1 {
                PlayerOption::Rock => PlayerOption::Scissors,
                PlayerOption::Paper => PlayerOption::Rock,
                PlayerOption::Scissors => PlayerOption::Paper,
            },
            MatchResult::Draw => match self.input_1 {
                PlayerOption::Rock => PlayerOption::Rock,
                PlayerOption::Paper => PlayerOption::Paper,
                PlayerOption::Scissors => PlayerOption::Scissors,
            },
        };
        player_choice.as_usize() + MatchResult::from_result(&self.input_2)
    }
}
