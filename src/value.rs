use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, parser::Stmt, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Bool(bool),

    // used for break
    Interrupted,
    Null,
    Function(Function),
    Builtin(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Box<Stmt>,
}


pub fn builtin_print(args: Vec<Value>) -> Value {
    for arg in args {
        print!("{:?} ", arg);
    }
    println!();
    Value::Null
}