use core::panic;
use std::{io::BufRead, thread::panicking};

use crate::token::{Token, TokenType, self};

pub fn tokenize_char(input: &mut &str) -> Option<Token> {
    let token = if let Some(c) = input.chars().next() {
        match c {
            '+' => Some(Token::new(TokenType::Plus, String::from("+"))),
            '-' => Some(Token::new(TokenType::Minus, String::from("-"))),
            '*' => Some(Token::new(TokenType::Mul, String::from("*"))),
            '/' => Some(Token::new(TokenType::Div, String::from("/"))),
            '(' => Some(Token::new(TokenType::LParen, String::from("("))),
            ')' => Some(Token::new(TokenType::RParen, String::from(")"))),
            '<' => Some(Token::new(TokenType::LessThan, String::from("<"))),
            '=' => Some(Token::new(TokenType::Assign, String::from("="))),
            '\n' => Some(Token::new(TokenType::NewLine, String::from("\n"))),
            ':' => Some(Token::new(TokenType::Colon, String::from(":"))),
            ',' => Some(Token::new(TokenType::Comma, String::from(","))),
            _ => None,
        }
    } else {
        None
    };
    if token.is_some() {
        *input = &input[1..];
    }
    token
}

pub fn tokenize_operator_multi_char(input: &mut &str) -> Option<Token> {
    let operator = ["=="];
    for op in operator {
        let token = if let Some(value) = match_pattern(input, op) {
            match value.as_str() {
                "==" => return Some(Token::new(TokenType::EqualEqual, value)),
                _ => {
                    panic!("Unknown operator: {}", value);
                }
            }
        } else {
            None
        };
        if token.is_some() {
            *input = &input[op.len()..];
        }
        return token;
    }
    None
}

pub fn skip_whitespace(input: &mut &str) -> usize {
    let pattern = r"^[ \t]+";
    let re = regex::Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(input) {
        let value = captures.get(0).unwrap().as_str();
        *input = &input[value.len()..];
        return value.len();
    }
    0
}

pub fn match_pattern(input: &mut &str, pattern: &str) -> Option<String> {
    let re = regex::Regex::new(pattern).unwrap();
    let value = if let Some(captures) = re.captures(input) {
        let value = captures.get(0).unwrap().as_str();
        *input = &input[value.len()..];
        Some(String::from(value))
    } else {
        None
    };
    value
}

fn tokenize_name(input: &mut &str) -> Option<Token> {
    let pattern = r"^[a-zA-Z_][a-zA-Z0-9_]*";
    if let Some(value) = match_pattern(input, pattern) {
        Some(Token::new(TokenType::Identifier, value))
    } else {
        None
    }
}

fn tokenize_keyword(input: &mut &str) -> Option<Token> {
    let keywords = vec!["def", "if", "else", "return", "while", "break", "print"];
    for keyword in keywords {
        if let Some(value) = match_pattern(input, keyword) {
            match value.as_str() {
                "def" => return Some(Token::new(TokenType::Def, value)),
                "if" => return Some(Token::new(TokenType::If, value)),
                "else" => return Some(Token::new(TokenType::Else, value)),
                "return" => return Some(Token::new(TokenType::Return, value)),
                "while" => return Some(Token::new(TokenType::While, value)),
                "break" => return Some(Token::new(TokenType::Break, value)),
                "print" => return Some(Token::new(TokenType::Print, value)),
                _ => {
                    panic!("Unknown keyword: {}", value);
                }
            }
        }
    }
    None
}

fn tokenize_int(input: &mut &str) -> Option<Token> {
    let pattern = r"^[0-9]+";

    if let Some(value) = match_pattern(input, pattern) {
        Some(Token::new(TokenType::Int, value))
    } else {
        None
    }
}

pub fn tokenize(lines: &Vec<String>) -> Vec<Token> {
    let mut indent_level = vec![0];
    let mut tokens = Vec::new();
    for line in lines {
        let mut input = &mut &line[..];
        let indent = skip_whitespace(&mut input);
        if indent > *indent_level.last().unwrap() {
            tokens.push(Token::new(TokenType::Indent, String::from("")));
            indent_level.push(indent);
        } else if indent < *indent_level.last().unwrap() {
            while indent < *indent_level.last().unwrap() {
                tokens.push(Token::new(TokenType::Dedent, String::from("")));
                indent_level.pop();
            }
        }
        loop {
            skip_whitespace(&mut input);
            if let Some(token) = tokenize_keyword(&mut input) {
                tokens.push(token);
            } else if let Some(token) = tokenize_name(&mut input) {
                tokens.push(token);
            } else if let Some(token) = tokenize_int(&mut input) {
                tokens.push(token);
            } else if let Some(token) = tokenize_operator_multi_char(input) {
                tokens.push(token);
            } else if let Some(token) = tokenize_char(&mut input) {
                tokens.push(token);
            } else {
                break;
            }
        }
        tokens.push(Token::new(TokenType::NewLine, String::from("\n")));
    }
    while indent_level.len() > 1 {
        tokens.push(Token::new(TokenType::Dedent, String::from("")));
        indent_level.pop();
    }

    tokens.push(Token::new(TokenType::EndMarker, String::from("")));
    tokens
}

pub fn read_lines(filename: &str) -> Vec<String> {
    // read file and return lines
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let lines = read_lines("tests/fib.py");

        let tokens = tokenize(&lines);
        for token in tokens {
            println!("{:?}", token);
        }
    }
    #[test]
    fn test_return() {
        let lines = read_lines("tests/return.py");

        let tokens = tokenize(&lines);
        for token in tokens {
            println!("{:?}", token);
        }
    }
    #[test]
    fn test_assign() {
        let lines = read_lines("tests/var.py");
        let tokens = tokenize(&lines);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_def_with_multiple_empty_lines() {
        let lines = read_lines("tests/def.py");
        let tokens = tokenize(&lines);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_while() {
        let lines = read_lines("tests/while.py");
        let tokens = tokenize(&lines);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_print() {
        let lines = read_lines("tests/print.py");
        let tokens = tokenize(&lines);
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
