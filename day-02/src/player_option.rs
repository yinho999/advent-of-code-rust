pub enum PlayerOption {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for PlayerOption {
    fn from(string: &str) -> Self {
        match string {
            "A" => PlayerOption::Rock,
            "B" => PlayerOption::Paper,
            "C" => PlayerOption::Scissors,
            "X" => PlayerOption::Rock,
            "Y" => PlayerOption::Paper,
            "Z" => PlayerOption::Scissors,
            _ => panic!("Invalid input"),
        }
    }
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
