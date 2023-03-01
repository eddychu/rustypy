// use crate::{
//     environment::Environment,
//     parser::{Expr, Stmt},
//     token::{Token, TokenType},
// };
// #[derive(Debug, Clone, PartialEq)]
// pub enum Value {
//     Int(i32),
//     Bool(bool),
//     Null,
//     Func(Function),
// }

// #[derive(Debug, Clone, PartialEq)]

// pub struct Function {
//     pub name: Token,
//     pub params: Vec<Expr>,
//     pub body: Vec<Stmt>,
//     pub env: Environment,
//     pub return_value: Box<Value>,
// }

// pub struct Interpreter {}

// impl Interpreter {
//     pub fn interprete_expr(&self, expr: &Expr, env: &mut Environment) -> Value {
//         match expr {
//             Expr::IntLiteral(value) => Value::Int(*value),
//             Expr::Assign(name, value) => {
//                 let value = self.interprete_expr(value, env);
//                 match &**name {
//                     Expr::Identifier(token) => env.assign(token.value.clone(), value.clone()),
//                     _ => panic!("Invalid assignment"),
//                 }
//                 Value::Null
//             }
//             Expr::BinaryOp(left, right, op) => {
//                 let left = self.interprete_expr(left, env);
//                 let right = self.interprete_expr(right, env);
//                 match (left, right) {
//                     (Value::Int(left), Value::Int(right)) => match op.token_type {
//                         TokenType::Plus => Value::Int(left + right),
//                         TokenType::Minus => Value::Int(left - right),
//                         TokenType::Mul => Value::Int(left * right),
//                         TokenType::Div => Value::Int(left / right),
//                         _ => panic!("Invalid binary operator"),
//                     },
//                     _ => panic!("Invalid binary expression"),
//                 }
//             }
//             Expr::Identifier(token) => {
//                 let value = env.get(&token.value);
//                 match value {
//                     Some(value) => value.clone(),
//                     None => panic!("Undefined variable"),
//                 }
//             }
//             Expr::Call(callee, args) => {
//                 let callee = self.interprete_expr(callee, env);
//                 let mut arguments = Vec::new();
//                 for arg in args {
//                     arguments.push(self.interprete_expr(arg, env));
//                 }
//                 match callee {
//                     Value::Func(function) => {
//                         let mut env = function.env;
//                         for (i, param) in function.params.iter().enumerate() {
//                             if let Expr::Identifier(token) = param {
//                                 env.define(token.value.clone(), arguments[i].clone());
//                             }
//                         }
//                         for stmt in &function.body {
//                             self.interprete_stmt(stmt, &mut env);
//                         }
//                         *function.return_value
//                     }
//                     _ => panic!("Invalid function call"),
//                 }
//             }
//             _ => panic!("Invalid expression"),
//         }
//     }

//     pub fn interprete(&self, statements: &Vec<Stmt>) -> Value {
//         let mut env = Environment::new();
//         env.define("return".to_string(), Value::Null);
//         for stmt in statements {
//             self.interprete_stmt(stmt, &mut env);
//         }
//         println!("{:?}", env);
//         env.get("return").unwrap().clone()
//     }

//     pub fn interprete_stmt(&self, stmt: &Stmt, env: &mut Environment) -> Value {
//         match stmt {
//             Stmt::Expr(expr) => self.interprete_expr(expr, env),
//             Stmt::Return(expr) => {
//                 let value = self.interprete_expr(expr, env);
//                 env.assign("return".to_string(), value);
//                 Value::Null
//             }
//             Stmt::Def(Expr::Identifier(name), params, body) => {
//                 let function = Function {
//                     name: name.clone(),
//                     params: params.clone(),
//                     body: body.clone(),
//                     env: Environment::new_with_enclosing(env.clone()),
//                     return_value: Box::new(Value::Null),
//                 };
//                 env.define(name.value.clone(), Value::Func(function));
//                 Value::Null
//             }
//             _ => panic!("Invalid statement"),
//         }
//     }
// }

// #[cfg(test)]

// mod tests {
//     use super::*;
//     use crate::{environment, parser::Parser, scanner::Scanner, token};

//     #[test]
//     fn test_interpreter_assign() {
//         let source = std::fs::read_to_string("tests/var.py").unwrap();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();
//         let mut parser = Parser::new(tokens);
//         let expr = parser.parse();
//         let interpreter = Interpreter {};
//         let value = interpreter.interprete(&expr);
//         println!("{:?}", value);
//     }
//     #[test]
//     fn test_interpreter_return() {
//         let source = std::fs::read_to_string("tests/return.py").unwrap();
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();
//         let mut parser = Parser::new(tokens);
//         let expr = parser.parse();
//         let interpreter = Interpreter {};
//         let value = interpreter.interprete(&expr);
//         println!("{:?}", value);
//     }

//     // #[test]
//     // fn test_interpreter_return() {
//     //     let source = "return 1 + 2 * 3".to_string();
//     //     let mut scanner = Scanner::new(source);
//     //     let tokens = scanner.scan_tokens();
//     //     let mut parser = Parser::new(tokens);
//     //     let stmt = parser.parse_stmt();
//     //     let interpreter = Interpreter {};
//     //     let value = interpreter.interprete_stmt(&stmt);
//     //     assert_eq!(value, Value::Int(7));
//     // }
// }
