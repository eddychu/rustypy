use crate::{scanner::Scanner, token::TokenType};

pub trait AstNode {}
#[derive(Debug)]
pub enum Expr {
    IntLiteral(i64),
    Identifier(String),
    BinaryOp(Box<Expr>, Box<Expr>, String),
}

impl AstNode for Expr {}
#[derive(Debug)]
pub enum Stmt {
    Def(Expr, Vec<String>, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    Return(Box<Expr>),
    Expr(Box<Expr>),
}

impl AstNode for Stmt {}

pub struct Parser {
    pub scanner: Scanner,
}

impl Parser {
    pub fn new(scanner: Scanner) -> Self {
        Self { scanner }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        loop {
            let token = self.scanner.next_token().unwrap();
            if token.token_type == TokenType::EndMarker {
                break;
            }
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }
        stmts
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        let token = self.scanner.next_token().unwrap();
        if token.token_type == TokenType::Def {
            let name = self.parse_expr();
            self.scanner.next_token().unwrap();
            let mut params = Vec::new();
            loop {
                let token = self.scanner.next_token().unwrap();
                if token.token_type == TokenType::RParen {
                    break;
                }
                if token.token_type == TokenType::Identifier {
                    params.push(token.value);
                }
            }
            self.scanner.next_token().unwrap();
            let mut body = Vec::new();
            loop {
                let token = self.scanner.next_token().unwrap();
                if token.token_type == TokenType::Dedent {
                    break;
                }
                /// temperary hack consume newline
                self.scanner.next_token().unwrap();

                let stmt = self.parse_stmt();
                body.push(stmt);
            }
            Stmt::Def(name, params, body)
        } else if token.token_type == TokenType::Return {
            let expr = self.parse_expr();
            Stmt::Return(Box::new(expr))
        } else if token.token_type == TokenType::If {
            // let left_paren = self.scanner.next_token().unwrap();
            let cond = self.parse_expr();
            //right_paren = self.scanner.next_token().unwrap();
            let colon = self.scanner.next_token().unwrap();
            let mut then_body = Vec::new();
            self.scanner.next_token().unwrap();
            loop {
                let token = self.scanner.next_token().unwrap();
                if token.token_type == TokenType::Dedent {
                    break;
                }
                /// temperary hack consume newline
                self.scanner.next_token().unwrap();

                let stmt = self.parse_stmt();
                then_body.push(stmt);
            }
            let mut else_body = Vec::new();
            return Stmt::If(cond, then_body, else_body);
        } else {
            let expr = self.parse_expr();
            Stmt::Expr(Box::new(expr))
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let token = self.scanner.next_token().unwrap();
        if token.token_type == TokenType::Int {
            Expr::IntLiteral(token.value.parse().unwrap())
        } else if token.token_type == TokenType::Identifier {
            Expr::Identifier(token.value)
        } else {
            let left = self.parse_expr();
            let token = self.scanner.next_token().unwrap();
            let op = token.value;
            let right = self.parse_expr();
            Expr::BinaryOp(Box::new(left), Box::new(right), op)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let source = std::fs::read_to_string("tests/fib.py").unwrap();
        let mut scanner = Scanner::new(source);
        let mut parser = Parser::new(scanner);
        let stmts = parser.parse_stmt();
        println!("{:?}", stmts);
    }
}
