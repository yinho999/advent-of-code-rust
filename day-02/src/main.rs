use std::fs;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn get_value(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn from_opponent_string(string: &str) -> RPS {
        match string {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Invalid input"),
        }
    }
    fn from_player_string(string: &str) -> RPS {
        match string {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Invalid input"),
        }
    }
}

trait Match {
    fn user_fight_opponent(&self, other: &RPS) -> usize;
    fn predict_user_hand_by_result(&self, match_result: EndOfRound) -> usize;
}

impl Match for RPS {
    fn user_fight_opponent(&self, other: &RPS) -> usize {
        let result = match self {
            RPS::Rock => match other {
                RPS::Rock => EndOfRound::Draw,
                RPS::Paper => EndOfRound::Lose,
                RPS::Scissors => EndOfRound::Win,
            },
            RPS::Paper => match other {
                RPS::Rock => EndOfRound::Win,
                RPS::Paper => EndOfRound::Draw,
                RPS::Scissors => EndOfRound::Lose,
            },
            RPS::Scissors => match other {
                RPS::Rock => EndOfRound::Lose,
                RPS::Paper => EndOfRound::Win,
                RPS::Scissors => EndOfRound::Draw,
            },
        };
        EndOfRound::from_result_to_score(result) + self.get_value()
    }

    // self is opponent, match result is the result for player
    fn predict_user_hand_by_result(&self, match_result: EndOfRound) -> usize {
        let user_hand = match match_result {
            EndOfRound::Win => match self {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            EndOfRound::Lose => match self {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            EndOfRound::Draw => match self {
                RPS::Rock => RPS::Rock,
                RPS::Paper => RPS::Paper,
                RPS::Scissors => RPS::Scissors,
            },
        };
        user_hand.get_value() + EndOfRound::from_result_to_score(match_result)
    }
}

#[derive(Clone, Copy)]
enum EndOfRound {
    Win,
    Lose,
    Draw,
}

impl EndOfRound {
    fn from_result_to_score(result: EndOfRound) -> usize {
        match result {
            EndOfRound::Win => 6,
            EndOfRound::Lose => 0,
            EndOfRound::Draw => 3,
        }
    }

    fn from_string_to_EOR(input: &str) -> Self {
        match input {
            "X" => EndOfRound::Lose,
            "Y" => EndOfRound::Draw,
            "Z" => EndOfRound::Win,
            _ => panic!("Invalid input, unable to convert to end of round"),
        }
    }
}

fn load_file(file_name: &str) -> Vec<String> {
    let file_string = fs::read_to_string(file_name).expect("Unable to read the file");
    file_string
        .split("\n")
        .map(|chunk| chunk.to_string())
        .collect()
}
fn part_one(file_name: &str) -> usize {
    let file = load_file(file_name);
    let mut score = 0;
    for line in file {
        let mut line = line.split(" ");
        let opponent = RPS::from_opponent_string(line.next().unwrap());
        let player = RPS::from_player_string(line.next().unwrap());
        score += player.user_fight_opponent(&opponent);
    }
    score
}

fn part_two(file_name: &str) -> usize {
    let file = load_file(file_name);
    let mut score = 0;
    for line in file {
        let mut line = line.split(" ");
        let opponent = RPS::from_opponent_string(line.next().unwrap());
        let match_result = EndOfRound::from_string_to_EOR(line.next().unwrap());
        score += opponent.predict_user_hand_by_result(match_result);
    }
    score
}

fn main() {
    let score = part_two("data.txt");
    println!("{}", score);
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let score = part_one("example.txt");
        assert_eq!(score, 15);
    }
}
