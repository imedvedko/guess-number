use guess_number_lib::{self, Checker, GameResult, Parser};

fn main() {
    println!("Guess the number!");

    let max_number = 100;
    let parser = Parser::new(max_number);
    let checker = Checker::new(max_number);

    while let GameResult::Lose(_) = round(&parser, &checker) {}
}

fn round<'a>(parser: &Parser, checker: &Checker) -> GameResult<'a> {
    let guess = receive_guess(&parser);
    println!("You guessed: {}", guess);

    let result = checker.check(guess);
    println!("{}", result.message());

    result
}

fn receive_guess(parser: &Parser) -> u8 {
    loop {
        println!("Please input your guess.");
        let received_string = guess_number_lib::read();
        if let Some(guess) = parser.parse_guess(received_string) {
            return guess;
        }
    }
}
