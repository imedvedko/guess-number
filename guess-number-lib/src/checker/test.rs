use super::{Checker, GameResult};

fn check(guess: u8, win: bool) {
    let checker = Checker { secret: 10 };

    let game_win = match checker.check(guess) {
        GameResult::Win(_) => true,
        GameResult::Lose(_) => false,
    };

    assert_eq!(win, game_win);
}

#[test]
fn less_test() {
    check(9, false);
}

#[test]
fn greater_test() {
    check(11, false);
}

#[test]
fn win_test() {
    check(10, true);
}
