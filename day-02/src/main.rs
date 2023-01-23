use day_02::{game::Game, load_file, match_result::MatchResult, player_option::PlayerOption};

fn part_one(file_name: &str) -> usize {
    let file = load_file(file_name);
    let mut score = 0;
    for line in file {
        let line = line
            .split(" ")
            .map(|chunk| chunk.to_string())
            .collect::<Vec<String>>();
        let game = Game::new(line[0].to_owned(), PlayerOption::from(line[1].as_str()));
        score += game.play();
    }
    score
}

fn part_two(file_name: &str) -> usize {
    let file = load_file(file_name);
    let mut score = 0;
    for line in file {
        let line = line
            .split(" ")
            .map(|chunk| chunk.to_string())
            .collect::<Vec<String>>();
        let game = Game::new(line[0].to_owned(), MatchResult::from(line[1].as_str()));
        score += game.play();
    }
    score
}

fn main() {
    let score = part_one("data.txt");
    println!("{}", score);
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

    #[test]
    fn test_part_two() {
        let score = part_two("example.txt");
        assert_eq!(score, 12);
    }
}
