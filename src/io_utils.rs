use console::Term;
use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

pub fn wait_for_key_press() {
    io::stdin().read_line(&mut String::new()).unwrap();
}

pub fn get_user_input_char() -> char {
    Term::stdout().read_char().unwrap()
}
