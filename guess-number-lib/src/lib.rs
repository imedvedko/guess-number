mod checker;
mod parser;

pub use {
    checker::{Checker, GameResult},
    parser::{read_console as read, Parser},
};
