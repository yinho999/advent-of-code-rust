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

impl From<&str> for MatchResult {
    fn from(string: &str) -> Self {
        match string {
            "X" => MatchResult::Lose,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            _ => panic!("Invalid input"),
        }
    }
}
