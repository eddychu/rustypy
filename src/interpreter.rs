use crate::{
    environment::Environment,
    parser::{Expr, Stmt},
    token::TokenType,
    value::{Function, Value},
};

fn interpret_expr(expr: &Expr, env: &mut Environment) -> Value {
    match expr {
        Expr::IntLiteral(value) => Value::Int(*value),
        Expr::BinaryOp(left, right, op) => {
            let left = interpret_expr(left, env);
            let right = interpret_expr(right, env);
            match (left, right) {
                (Value::Int(left), Value::Int(right)) => match op.token_type {
                    TokenType::Plus => Value::Int(left + right),
                    TokenType::Minus => Value::Int(left - right),
                    TokenType::Mul => Value::Int(left * right),
                    TokenType::Div => Value::Int(left / right),
                    _ => panic!("Unknown operator"),
                },
                _ => panic!("Invalid operands"),
            }
        }
        Expr::Assign(name, value) => {
            let value = interpret_expr(value, env);
            match name.as_ref() {
                Expr::Identifier(token) => env.assign(token.value.clone(), value.clone()),
                _ => panic!("Invalid assignment"),
            }
            Value::Null
        }
        _ => panic!("Unknown expression"),
    }
}

fn interpret_stmt(stmt: &Stmt, env: &mut Environment) -> Value {
    match stmt {
        Stmt::Expr(expr) => {
            let value = interpret_expr(expr, env);
            value
        }
        Stmt::Def(name, args, body) => {
            let new_env = Environment::new_with_enclosing(env.clone());
            match name {
                Expr::Identifier(token) => {
                    let func = Value::Function(Function {
                        name: token.value.clone(),
                        args: args
                            .iter()
                            .map(|arg| match arg {
                                Expr::Identifier(token) => token.value.clone(),
                                _ => panic!("Invalid argument"),
                            })
                            .collect(),
                        body: body.clone(),
                        env: new_env,
                    });
                    env.define(token.value.clone(), func);
                }
                _ => panic!("Invalid function name"),
            }

            Value::Null
        }
        _ => panic!("Unknown statement"),
    }
}

#[cfg(test)]

mod tests {
    use crate::{
        parser::{parse, parse_program},
        tokenizer::{read_lines, tokenize},
    };

    use super::*;

    #[test]
    fn test_assign() {
        let mut env = Environment::new();
        let source = read_lines("tests/var.py");
        let tokens = tokenize(&source);
        let stmt = parse(tokens);
        for stmt in stmt {
            println!("{:?}", interpret_stmt(&stmt, &mut env));
        }
    }
}
