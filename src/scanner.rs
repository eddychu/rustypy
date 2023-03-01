use crate::token::{Token, TokenType};
pub struct Scanner {
    source: String,
    pos: usize,
    new_line: bool,
    indent_stack: Vec<u32>,
    current_indent: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            pos: 0,
            new_line: true,
            indent_stack: vec![0],
            current_indent: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.unwrap());
            if tokens.last().unwrap().token_type == TokenType::EndMarker {
                break;
            }
        }
        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let spaces = self.skip_whitespace();
        if self.new_line {
            self.current_indent = spaces;
        }
        if self.current_indent > *self.indent_stack.last().unwrap() {
            self.indent_stack.push(self.current_indent);
            self.new_line = false;
            return Some(Token {
                token_type: TokenType::Indent,
                value: "".to_string(),
            });
        } else if self.current_indent < *self.indent_stack.last().unwrap() {
            self.indent_stack.pop();
            self.new_line = false;
            return Some(Token {
                token_type: TokenType::Dedent,
                value: "".to_string(),
            });
        }
        if let Some(c) = self.peek_char() {
            self.new_line = false;
            match c {
                '+' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Plus,
                        value: "+".to_string(),
                    });
                }
                '-' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Minus,
                        value: "-".to_string(),
                    });
                }
                '*' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Mul,
                        value: "*".to_string(),
                    });
                }
                '/' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Div,
                        value: "/".to_string(),
                    });
                }
                ':' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::Colon,
                        value: ":".to_string(),
                    });
                }
                '(' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::LParen,
                        value: "(".to_string(),
                    });
                }
                ')' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::RParen,
                        value: ")".to_string(),
                    });
                }
                '<' => {
                    self.next_char();
                    return Some(Token {
                        token_type: TokenType::LessThan,
                        value: "<".to_string(),
                    });
                }

                '\n' => {
                    self.next_char();
                    self.new_line = true;
                    return Some(Token {
                        token_type: TokenType::NewLine,
                        value: "\n".to_string(),
                    });
                }
                '0'..='9' => {
                    let value = self.take_while(|c| c.is_digit(10));
                    return Some(Token {
                        token_type: TokenType::Int,
                        value,
                    });
                }
                'a'..='z' | 'A'..='Z' => {
                    let value = self.take_name();

                    if value == "def" {
                        return Some(Token {
                            token_type: TokenType::Def,
                            value,
                        });
                    }

                    if value == "if" {
                        return Some(Token {
                            token_type: TokenType::If,
                            value,
                        });
                    }

                    if value == "return" {
                        return Some(Token {
                            token_type: TokenType::Return,
                            value,
                        });
                    }

                    return Some(Token {
                        token_type: TokenType::Identifier,
                        value,
                    });
                }
                // handle end of file
                '\0' => {
                    return Some(Token {
                        token_type: TokenType::EndMarker,
                        value: "".to_string(),
                    });
                }

                _ => {
                    println!("Unexpected character: {}", c);
                    todo!()
                }
            }
        }
        return Some(Token {
            token_type: TokenType::EndMarker,
            value: "".to_string(),
        });
    }

    pub fn next_char(&mut self) -> Option<char> {
        if self.pos < self.source.len() {
            let c = self.source[self.pos..].chars().next().unwrap();
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        if self.pos < self.source.len() {
            Some(self.source[self.pos..].chars().next().unwrap())
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) -> u32 {
        let mut count = 0;
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() && c != '\n' {
                self.next_char();
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn take_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let c: String = self
            .source
            .chars()
            .skip(self.pos)
            .take_while(|c| f(*c))
            .collect();
        self.pos += c.len();
        c
    }

    pub fn take_name(&mut self) -> String {
        // the first character must be of char
        let c: String = self
            .source
            .chars()
            .skip(self.pos)
            .take_while(|c| c.is_alphanumeric())
            .collect();
        self.pos += c.len();
        c
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let source = "1 + 2".to_string();
        let mut scanner = Scanner::new(source);
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Int);
        assert_eq!(token.value, "1".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Int);
        assert_eq!(token.value, "2".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::EndMarker);
        assert_eq!(token.value, "".to_string());
    }

    #[test]
    fn test_read_file() {
        let source = std::fs::read_to_string("tests/fib.py").unwrap();
        let mut scanner = Scanner::new(source);
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Def);
        assert_eq!(token.value, "def".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "fib".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::LParen);
        assert_eq!(token.value, "(".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "x".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::RParen);
        assert_eq!(token.value, ")".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Colon);
        assert_eq!(token.value, ":".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NewLine);
        assert_eq!(token.value, "\n".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Indent);
        assert_eq!(token.value, "".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::If);
        assert_eq!(token.value, "if".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "x".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::LessThan);
        assert_eq!(token.value, "<".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Int);
        assert_eq!(token.value, "2".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Colon);
        assert_eq!(token.value, ":".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NewLine);
        assert_eq!(token.value, "\n".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Indent);
        assert_eq!(token.value, "".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Return);
        assert_eq!(token.value, "return".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "x".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NewLine);
        assert_eq!(token.value, "\n".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Dedent);
        assert_eq!(token.value, "".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Return);
        assert_eq!(token.value, "return".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "fib".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::LParen);
        assert_eq!(token.value, "(".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "x".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Minus);
        assert_eq!(token.value, "-".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Int);
        assert_eq!(token.value, "1".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::RParen);
        assert_eq!(token.value, ")".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "fib".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::LParen);
        assert_eq!(token.value, "(".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "x".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Minus);
        assert_eq!(token.value, "-".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Int);
        assert_eq!(token.value, "2".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::RParen);
        assert_eq!(token.value, ")".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::NewLine);
        assert_eq!(token.value, "\n".to_string());
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::EndMarker);
        assert_eq!(token.value, "".to_string());
    }
}
