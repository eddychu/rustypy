use core::panic;

use crate::{
    scanner::Scanner,
    token::{Token, TokenType},
};

pub trait AstNode {}
#[derive(Debug)]
pub enum Expr {
    IntLiteral(i32),
    Identifier(Token),
    BinaryOp(Box<Expr>, Box<Expr>, Token),
    Call(Box<Expr>, Vec<Expr>),
}

impl AstNode for Expr {}
#[derive(Debug)]
pub enum Stmt {
    Def(Expr, Vec<Expr>, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    Return(Box<Expr>),
    Expr(Box<Expr>),
    Invalid,
}

impl AstNode for Stmt {}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        loop {
            if self.peek().token_type == TokenType::EndMarker {
                break;
            }
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }
        stmts
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        let token = self.peek();
        if token.token_type == TokenType::Return {
            return self.parse_return();
        }
        if token.token_type == TokenType::Def {
            return self.parse_def();
        }

        if token.token_type == TokenType::If {
            return self.parse_if();
        }
        Stmt::Invalid
    }

    pub fn parse_def(&mut self) -> Stmt {
        self.consume(TokenType::Def, "Expect 'def' keyword");
        let name = self.parse_identifier();
        self.consume(TokenType::LParen, "Expect '('");
        let mut params = Vec::new();
        while !self.is_match(vec![TokenType::RParen]) {
            let param = self.parse_identifier();
            params.push(Expr::Identifier(param));
            if self.is_match(vec![TokenType::Comma]) {
                // self.consume(TokenType::Comma, "Expect ','");
            }
        }
        self.consume(TokenType::Colon, "Expect ':'");
        let body = self.parse_block();
        // handle new line and deden
        Stmt::Def(Expr::Identifier(name), params, body)
    }

    pub fn parse_if(&mut self) -> Stmt {
        self.consume(TokenType::If, "Expect 'if' keyword");
        let condition = self.parse_expr();
        self.consume(TokenType::Colon, "Expect ':'");

        let then_branch = self.parse_block();
        let mut else_branch = Vec::new();
        if self.is_match(vec![TokenType::Else]) {
            self.consume(TokenType::Colon, "Expect ':'");
            else_branch = self.parse_block();
        }
        Stmt::If(condition, then_branch, else_branch)
    }

    pub fn parse_return(&mut self) -> Stmt {
        self.consume(TokenType::Return, "Expect 'return' keyword");
        let value = self.parse_expr();
        Stmt::Return(Box::new(value))
    }

    pub fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.consume(TokenType::NewLine, "Expect newline");
        self.consume(TokenType::Indent, "Expect indent");
        while !self.is_match(vec![TokenType::Dedent]) {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
            if self.is_match(vec![TokenType::NewLine]) {}
        }
        // self.consume(TokenType::NewLine, "Expect newline");
        // self.consume(TokenType::Dedent, "Expect dedent");
        stmts
    }

    pub fn parse_identifier(&mut self) -> Token {
        let token = self.peek();
        if token.token_type == TokenType::Identifier {
            self.advance();
            token
        } else {
            panic!("Expect identifier");
        }
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_equality()
    }

    pub fn parse_equality(&mut self) -> Expr {
        let expr = self.parse_comparison();
        // while self.is_match(vec![TokenType::LessThan]) {
        //     let operator = self.previous();
        //     let right = self.parse_comparison();
        //     expr = Expr::BinaryOp(Box::new(expr), Box::new(right), operator.value);
        // }
        expr
    }

    pub fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();
        while self.is_match(vec![TokenType::LessThan]) {
            let operator = self.previous();
            let right = self.parse_term();
            expr = Expr::BinaryOp(Box::new(expr), Box::new(right), operator);
        }
        expr
    }

    pub fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        while self.is_match(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.parse_factor();
            expr = Expr::BinaryOp(Box::new(expr), Box::new(right), operator);
        }
        expr
    }

    pub fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        while self.is_match(vec![TokenType::Mul, TokenType::Div]) {
            let operator = self.previous();
            let right = self.parse_unary();
            expr = Expr::BinaryOp(Box::new(expr), Box::new(right), operator);
        }
        expr
    }

    pub fn parse_unary(&mut self) -> Expr {
        if self.is_match(vec![TokenType::Minus]) {
            let operator = self.previous();
            let right = self.parse_unary();
            Expr::BinaryOp(Box::new(Expr::IntLiteral(0)), Box::new(right), operator)
        } else {
            self.parse_call()
        }
    }

    pub fn parse_call(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        while self.is_match(vec![TokenType::LParen]) {
            let mut args = Vec::new();
            while !self.is_match(vec![TokenType::RParen]) {
                let arg = self.parse_expr();
                args.push(arg);
                if self.is_match(vec![TokenType::Comma]) {
                    // self.consume(TokenType::Comma, "Expect ','");
                }
            }
            expr = Expr::Call(Box::new(expr), args);
        }
        expr
    }

    pub fn parse_primary(&mut self) -> Expr {
        if self.is_match(vec![TokenType::Int]) {
            Expr::IntLiteral(self.previous().value.parse().unwrap())
        } else if self.is_match(vec![TokenType::Identifier]) {
            Expr::Identifier(self.previous())
        } else if self.is_match(vec![TokenType::LParen]) {
            let expr = self.parse_expr();
            self.consume(TokenType::RParen, "Expect ')' after expression.");
            expr
        } else {
            panic!("Expect expression.");
        }
    }

    fn is_match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EndMarker
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
        // let source = std::fs::read_to_string("tests/fib.py").unwrap();
        let source = "1 + 2 * 3".to_string();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();
        println!("{:?}", expr);
    }

    #[test]
    fn test_parse() {
        let source = std::fs::read_to_string("tests/fib.py").unwrap();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();
        print!("{:?}", stmt)
    }
    #[test]
    fn test_return_stmt() {
        let source = "return 1 + 2 * 3".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_stmt();
        println!("{:?}", expr);
    }
    #[test]
    fn test_parse_block() {
        let source = "\n    return 1\n".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_block();
        println!("{:?}", expr);
    }
    #[test]
    fn test_def_stmt() {
        let source = "def fib(n):\n    return 1\n".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_def();
        println!("{:?}", expr);
    }
    #[test]
    fn test_if_stmt() {
        let source = "if n < 1:\n    return 1\n".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_if();
        println!("{:?}", expr);
    }
}
