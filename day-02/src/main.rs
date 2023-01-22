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
    fn play(&self, other: &RPS) -> usize;
}

impl Match for RPS {
    fn play(&self, other: &RPS) -> usize {
        let result = match self {
            RPS::Rock => match other {
                RPS::Rock => Result::Draw,
                RPS::Paper => Result::Lose,
                RPS::Scissors => Result::Win,
            },
            RPS::Paper => match other {
                RPS::Rock => Result::Win,
                RPS::Paper => Result::Draw,
                RPS::Scissors => Result::Lose,
            },
            RPS::Scissors => match other {
                RPS::Rock => Result::Lose,
                RPS::Paper => Result::Win,
                RPS::Scissors => Result::Draw,
            },
        };
        Result::from_result(result) + self.get_value()
    }
}

enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn from_result(result: Result) -> usize {
        match result {
            Result::Win => 6,
            Result::Lose => 0,
            Result::Draw => 3,
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
        score += player.play(&opponent);
    }
    score
}

fn main() {}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score() {
        let score = part_one("example.txt");
        assert_eq!(score, 15);
    }
}
