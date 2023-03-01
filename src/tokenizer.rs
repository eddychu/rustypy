use core::panic;
use std::thread::panicking;

use crate::token::{Token, TokenType};

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

pub fn tokenize_name(input: &mut &str) -> Option<Token> {
    let pattern = r"^[a-zA-Z_][a-zA-Z0-9_]*";
    return tokenize_pattern(input, pattern);
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

pub fn tokenize_int(input: &mut &str) -> Option<Token> {
    let pattern = r"^[0-9]+";
    return tokenize_pattern(input, pattern);
}

pub fn tokenize_pattern(input: &mut &str, pattern: &str) -> Option<Token> {
    let re = regex::Regex::new(pattern).unwrap();
    let token = if let Some(captures) = re.captures(input) {
        let value = captures.get(0).unwrap().as_str();
        *input = &input[value.len()..];

        if value == "def" {
            return Some(Token::new(TokenType::Def, String::from(value)));
        }
        if value == "if" {
            return Some(Token::new(TokenType::If, String::from(value)));
        }

        if value == "else" {
            return Some(Token::new(TokenType::Else, String::from(value)));
        }

        if value == "return" {
            return Some(Token::new(TokenType::Return, String::from(value)));
        }

        Some(Token::new(TokenType::Identifier, String::from(value)))
    } else {
        None
    };
    token
}

pub fn generate_indent(input: &mut &str) -> Option<Token> {
    let pattern = r"^[ \t]+";
    let re = regex::Regex::new(pattern).unwrap();
    let token = if let Some(captures) = re.captures(input) {
        let value = captures.get(0).unwrap().as_str();
        *input = &input[value.len()..];
        Some(Token::new(TokenType::Indent, String::from(value)))
    } else {
        None
    };
    token
}

pub fn tokenize_line(line: &str, indent_level: &mut Vec<usize>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut input = line;
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
        if let Some(token) = tokenize_char(&mut input) {
            tokens.push(token);
        } else if let Some(token) = tokenize_int(&mut input) {
            tokens.push(token);
        } else if let Some(token) = tokenize_name(&mut input) {
            tokens.push(token);
        } else {
            break;
        }
    }
    tokens.push(Token::new(TokenType::NewLine, String::from("\n")));
    tokens
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let lines = source.split('\n').collect::<Vec<&str>>();
    let mut tokens = Vec::new();

    let mut indent_level = vec![0];

    for line in lines {
        let line_tokens = tokenize_line(line, &mut indent_level);
        tokens.extend(line_tokens);
    }
    tokens.push(Token::new(TokenType::EndMarker, String::from("")));
    tokens
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let source = std::fs::read_to_string("tests/fib.py").unwrap();
        let tokens = tokenize(&source);
        for token in tokens {
            println!("{:?}", token);
        }
    }
    #[test]
    fn test_return() {
        let source = std::fs::read_to_string("tests/return.py").unwrap();
        let tokens = tokenize(&source);
        for token in tokens {
            println!("{:?}", token);
        }
    }
    #[test]
    fn test_assign() {
        let source = std::fs::read_to_string("tests/var.py").unwrap();
        let tokens = tokenize(&source);
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
