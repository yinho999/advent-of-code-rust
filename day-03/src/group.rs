use crate::rucksack::RustStackError;

pub struct Group {
    elf_one: String,
    elf_two: String,
    elf_three: String,
}

impl Group {
    pub fn new(elf_one: String, elf_two: String, elf_three: String) -> Group {
        Group {
            elf_one,
            elf_two,
            elf_three,
        }
    }
    pub fn find_common(&self) -> Result<char, RustStackError> {
        for i in self.elf_one.as_bytes() {
            match self.elf_two.find(*i as char) {
                Some(i) => {
                    let answer = self.elf_two.as_bytes()[i] as char;
                    match self.elf_three.find(answer) {
                        Some(i) => {
                            let answer = self.elf_three.as_bytes()[i] as char;
                            return Ok(answer);
                        }
                        None => continue,
                    }
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
