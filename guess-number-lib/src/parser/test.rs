use super::Parser;

fn check(received_string: &str, expected_value: Option<u8>) {
    let parser = Parser { max_number: 100 };

    assert_eq!(
        expected_value,
        parser.parse_guess(String::from(received_string))
    );
}

#[test]
fn less_test() {
    check("0", None);
}

#[test]
fn greater_test() {
    check("101", None);
}

#[test]
fn less_than_zero_test() {
    check("-1", None);
}

#[test]
fn word_test() {
    check("test", None);
}

#[test]
fn min_test() {
    check("1", Some(1));
}

#[test]
fn max_test() {
    check("100", Some(100));
}
