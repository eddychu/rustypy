use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, parser::Stmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Null,
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Box<Stmt>,
}
