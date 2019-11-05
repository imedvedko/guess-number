#[cfg(test)]
mod test;

use {rand::Rng, std::cmp::Ordering};

pub enum GameResult<'a> {
    Win(&'a str),
    Lose(&'a str),
}

impl<'a> GameResult<'a> {
    pub fn message(&self) -> &str {
        match self {
            GameResult::Win(message) => message,
            GameResult::Lose(message) => message,
        }
    }
}

pub struct Checker {
    secret: u8,
}

impl Checker {
    pub fn new(max_number: u8) -> Self {
        Self {
            secret: rand::thread_rng().gen_range(0, max_number) + 1,
        }
    }

    pub fn check<'a>(&self, guess: u8) -> GameResult<'a> {
        match guess.cmp(&self.secret) {
            Ordering::Less => GameResult::Lose("Too small!"),
            Ordering::Greater => GameResult::Lose("Too big!"),
            Ordering::Equal => GameResult::Win("You win!"),
        }
    }
}
