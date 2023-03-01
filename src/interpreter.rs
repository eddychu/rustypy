use crate::{
    parser::Expr,
    token::{Token, TokenType},
};
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interprete_expr(&self, expr: &Expr) -> Value {
        match expr {
            Expr::IntLiteral(value) => Value::Int(*value),
            Expr::BinaryOp(left, right, op) => {
                let left = self.interprete_expr(left);
                let right = self.interprete_expr(right);
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => match op.token_type {
                        TokenType::Plus => Value::Int(left + right),
                        TokenType::Minus => Value::Int(left - right),
                        TokenType::Mul => Value::Int(left * right),
                        TokenType::Div => Value::Int(left / right),
                        _ => panic!("Invalid binary operator"),
                    },
                    _ => panic!("Invalid binary expression"),
                }
            }
            _ => panic!("Invalid expression"),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::{parser::Parser, scanner::Scanner, token};

    #[test]
    fn test_interpreter() {
        let source = "1 + 2 * 3".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();
        let interpreter = Interpreter {};
        let value = interpreter.interprete_expr(&expr);
        assert_eq!(value, Value::Int(7));
    }

    
}
