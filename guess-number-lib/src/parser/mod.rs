#[cfg(test)]
mod test;

use std::io;

pub struct Parser {
    max_number: u8,
}

impl Parser {
    pub fn new(max_number: u8) -> Self {
        Self { max_number }
    }

    pub fn parse_guess(&self, guess: String) -> Option<u8> {
        match guess.trim().parse() {
            Ok(num) if num > 0 && num <= self.max_number => Some(num),
            _ => None,
        }
    }
}

pub fn read_console() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess
}
