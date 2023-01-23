use std::fs;

pub mod game;
pub mod match_result;
pub mod player_option;
pub fn load_file(file_name: &str) -> Vec<String> {
    let file_string = fs::read_to_string(file_name).expect("Unable to read the file");
    file_string
        .split("\n")
        .map(|chunk| chunk.to_string())
        .collect()
}
