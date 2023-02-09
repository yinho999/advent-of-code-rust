use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct Ruststack {
    compartment_one: String,
    compartment_two: String,
}

impl FromStr for Ruststack {
    type Err = RustStackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 1 {
            return Err(RustStackError::ParseStringError);
        }
        let (compartment_one, compartment_two) = s.split_at(s.len() / 2);
        Ok(Ruststack {
            compartment_one: compartment_one.to_string(),
            compartment_two: compartment_two.to_string(),
        })
    }
}

impl Ruststack {
    pub fn find_common(&self) -> Result<char, RustStackError> {
        for i in self.compartment_one.as_bytes() {
            match self.compartment_two.find(*i as char) {
                Some(i) => {
                    let answer = self.compartment_two.as_bytes()[i] as char;
                    return Ok(answer);
                }
                None => continue,
            }
        }
        Err(RustStackError::UnableToFindCommonChar)
    }
    pub fn return_priority(&self) -> u8 {
        let character = self.find_common().unwrap();
        if character.is_ascii_lowercase() {
            (character as u8) - 96
        } else {
            (character as u8) - 64 + 26
        }
    }
}

#[derive(Debug, Clone)]
pub enum RustStackError {
    ParseStringError,
    UnableToFindCommonChar,
}

impl Display for RustStackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustStackError::ParseStringError => write!(f, "Unable to parse String"),
            RustStackError::UnableToFindCommonChar => {
                write!(f, "Unable to find string in common")
            }
        }
    }
}
