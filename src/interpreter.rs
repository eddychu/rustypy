
use crate::{
    environment::Environment,
    parser::{Expr, Stmt},
    token::TokenType,
    value::{Function, Value},
};

fn interpret_expr(expr: &Expr, env: &mut Environment, frame_index: usize) -> Value {
    match expr {
        Expr::IntLiteral(value) => Value::Int(*value),
        Expr::BinaryOp(left, right, op) => {
            let left = interpret_expr(left, env, frame_index);
            let right = interpret_expr(right, env, frame_index);
            match (left, right) {
                (Value::Int(left), Value::Int(right)) => match op.token_type {
                    TokenType::Plus => Value::Int(left + right),
                    TokenType::Minus => Value::Int(left - right),
                    TokenType::Mul => Value::Int(left * right),
                    TokenType::Div => Value::Int(left / right),
                    TokenType::LessThan => Value::Bool(left < right),
                    TokenType::EqualEqual => Value::Bool(left == right),
                    _ => panic!("Unknown operator"),
                },
                _ => panic!("Invalid operands"),
            }
        }
        Expr::Assign(name, value) => {
            let value = interpret_expr(value, env, frame_index);
            match name.as_ref() {
                Expr::Identifier(token) => {
                    env.set(&token.value, value, frame_index);
                }
                _ => panic!("Invalid assignment"),
            }
            Value::Null
        }
        Expr::Call(name, args) => {
            let func = match name.as_ref() {
                Expr::Identifier(token) => match env.get(&token.value, frame_index) {
                    Some(Value::Function(func)) => func.clone(),
                    _ => panic!("Invalid function"),
                },
                _ => panic!("Invalid function"),
            };
            let new_index = env.allocate_new_frame();
            env.envs[new_index].parent = Some(frame_index);
            for (i, arg) in args.iter().enumerate() {
                let value = interpret_expr(arg, env, new_index);
                env.set(&func.args[i], value, new_index);
            }
            interpret_stmt(&func.body, env, new_index)
        }
        Expr::Identifier(token) => match env.get(&token.value, frame_index) {
            Some(value) => value.clone(),
            None => panic!("Undefined variable"),
        },
        _ => panic!("Unknown expression"),
    }
}

fn interpret_stmt(stmt: &Stmt, env: &mut Environment, frame_index: usize) -> Value {
    match stmt {
        Stmt::Expr(expr) => {
            let value = interpret_expr(expr, env, frame_index);
            value
        }
        Stmt::Def(name, args, body) => {
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
                    });
                    env.set(&token.value, func, frame_index);
                }
                _ => panic!("Invalid function name"),
            }

            Value::Null
        }
        Stmt::Return(expr) => interpret_expr(expr, env, frame_index),
        Stmt::Block(stmts) => {
            let mut value = Value::Null;
            let new_frame_index = env.allocate_new_frame();
            env.envs[new_frame_index].parent = Some(frame_index);
            for stmt in stmts {
                value = interpret_stmt(stmt, env, new_frame_index);
                if value != Value::Null {
                    break;
                }
            }
            value
        }
        Stmt::If(condition, then_branch, else_branch) => {
            let condition = interpret_expr(condition, env, frame_index);
            match condition {
                Value::Bool(true) => interpret_stmt(then_branch, env, frame_index),
                Value::Bool(false) => match else_branch {
                    Some(else_branch) => interpret_stmt(else_branch, env, frame_index),
                    None => Value::Null,
                },
                _ => panic!("Invalid condition"),
            }
        }
        Stmt::While(condition, body) => {
            let mut value = Value::Null;
            loop {
                let condition = interpret_expr(condition, env, frame_index);
                match condition {
                    Value::Bool(true) => {
                        value = interpret_stmt(body, env, frame_index);

                        if value != Value::Null  || value == Value::Interrupted{
                            break;
                        }
                    }
                    Value::Bool(false) => break,
                    _ => panic!("Invalid condition"),
                }
            }
            if value == Value::Interrupted{
                Value::Null
            } else {
                value
            }
        }
        _ => {
            println!("{:?}", stmt);
            panic!("Unknown statement")
        }
    }
}

#[cfg(test)]

mod tests {
    use crate::{
        parser::parse,
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
            println!("{:?}", interpret_stmt(&stmt, &mut env, 0));
            println!("{}", env);
        }
    }

    #[test]
    fn test_def_and_all() {
        let mut env = Environment::new();
        let source = read_lines("tests/def.py");
        let tokens = tokenize(&source);
        let stmt = parse(tokens);
        for stmt in stmt {
            println!("{:?}", interpret_stmt(&stmt, &mut env, 0));
            println!("{}", env);
        }
    }

    #[test]
    fn test_fib() {
        let mut env = Environment::new();
        let source = read_lines("tests/fib.py");
        let tokens = tokenize(&source);
        let stmt = parse(tokens);
        for stmt in stmt {
            println!("{:?}", interpret_stmt(&stmt, &mut env, 0));
            println!("{}", env);
        }
    }

    #[test]
    fn test_if() {
        let mut env = Environment::new();
        let source = read_lines("tests/while.py");
        let tokens = tokenize(&source);
        let stmt = parse(tokens);
        for stmt in stmt {
            println!("{:?}", interpret_stmt(&stmt, &mut env, 0));
            println!("{}", env);
        }
    }
}
