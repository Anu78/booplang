mod lexer;
use lexer::get_token;
use std::fs;

fn main() {
    let path = std::env::args().nth(1);

    if path.is_none() {
        println!("usage: boop <file path>");
        return;
    }
    let path = path.unwrap();

    let file_string =
        fs::read_to_string(path).expect("unable to read provided file or file does not exist.");

    println!("{file_string}");

    let tokens = get_token(file_string);

    for token in tokens.iter() {
        println!("{token}");
    }
}
