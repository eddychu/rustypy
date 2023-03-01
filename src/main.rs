mod interpreter;
mod parser;
mod scanner;
mod token;
use scanner::Scanner;
fn main() {
    // is '\n' considered a whitespace?
    let source = std::fs::read_to_string("tests/fib.py").unwrap();
    let mut scanner = Scanner::new(source);

    loop {
        let token = scanner.next_token().unwrap();
        println!("{:?}", token);
        if token.token_type == token::TokenType::EndMarker {
            break;
        }
    }
}
