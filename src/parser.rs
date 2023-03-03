use core::panic;

use crate::token::{Token, TokenType};

pub trait AstNode: PrettyPrint {}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntLiteral(i32),
    Identifier(Token),
    BinaryOp(Box<Expr>, Box<Expr>, Token),
    Call(Box<Expr>, Vec<Expr>),
    Assign(Box<Expr>, Box<Expr>),
}

impl AstNode for Expr {} 
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Def(Expr, Vec<Expr>, Box<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Return(Expr),
    Print(Expr),
    Expr(Expr),
    Block(Vec<Stmt>),
    Break,
    While(Expr, Box<Stmt>),
}
impl AstNode for Stmt {}

pub trait PrettyPrint {
    fn pretty_print(&self, indent: usize) -> String;
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Expr::IntLiteral(value) => format!("{}", value),
            Expr::Identifier(token) => format!("{}", token.value),
            Expr::BinaryOp(left, right, op) => {
                format!(
                    "({} {} {})",
                    left.pretty_print(indent),
                    op.value,
                    right.pretty_print(indent)
                )
            }
            Expr::Call(callee, args) => {
                let mut s = format!("{}(", callee.pretty_print(indent));
                for (i, arg) in args.iter().enumerate() {
                    s.push_str(&arg.pretty_print(indent));
                    if i < args.len() - 1 {
                        s.push_str(", ");
                    }
                }
                s.push_str(")");
                s
            }
            Expr::Assign(name, value) => format!(
                "{} = {}",
                name.pretty_print(indent),
                value.pretty_print(indent)
            ),
        }
    }
}

impl PrettyPrint for Stmt {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Stmt::Def(name, params, body) => {
                let mut s = format!("def {}(", name.pretty_print(indent));
                for (i, param) in params.iter().enumerate() {
                    s.push_str(&param.pretty_print(indent));
                    if i < params.len() - 1 {
                        s.push_str(", ");
                    }
                }
                s.push_str("):\n");
                s.push_str(&body.pretty_print(indent));
                s
            }
            Stmt::If(condition, then_branch, else_branch) => {
                let mut s = format!(
                    "if {} :\n{}",
                    condition.pretty_print(indent),
                    then_branch.pretty_print(indent)
                );
                if let Some(else_branch) = else_branch {
                    s.push_str(&format!(" else {}", else_branch.pretty_print(indent)));
                }
                s
            }
            Stmt::Return(value) => {
                format!("return {}\n", value.pretty_print(indent))
            }
            Stmt::Expr(expr) => format!("{}", expr.pretty_print(indent)),
            Stmt::Block(stmts) => {
                let mut s = String::new();
                for stmt in stmts {
                    s.push_str(&format!(
                        "{}{}",
                        "    ".repeat(indent + 1),
                        stmt.pretty_print(indent + 1)
                    ));
                }
                s.push('\n');
                s
            }
            Stmt::Break => format!("{}break\n\n", "    ".repeat(indent)),
            Stmt::While(condition, body) => format!(
                "while {}:\n{}",
                condition.pretty_print(indent),
                body.pretty_print(indent)
            ),
            Stmt::Print(value) => format!("{}print {}\n","    ".repeat(indent), value.pretty_print(indent)),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::IntLiteral(value) => write!(f, "{}", value),
            Expr::Identifier(token) => write!(f, "{}", token.value),
            Expr::BinaryOp(left, right, op) => write!(f, "({} {} {})", left, op.value, right),
            Expr::Call(callee, args) => {
                write!(f, "{}(", callee)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
                Ok(())
            }
            Expr::Assign(name, value) => write!(f, "{} = {}", name, value),
        }
    }
}

// implement display for Stmt
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Def(name, params, body) => {
                write!(f, "def {}(", name)?;
                for (i, param) in params.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if i < params.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "):\n")?;
                write!(f, "\t{}", body)
            }
            Stmt::If(condition, body, else_body) => {
                write!(f, "if {}:\n", condition)?;
                write!(f, "{}", body)?;
                if let Some(else_body) = else_body {
                    write!(f, "else {}", else_body)?;
                }
                Ok(())
            }
            Stmt::Return(expr) => write!(f, "return {}", expr),
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::Block(stmts) => {
                // add tab indent
                for stmt in stmts {
                    write!(f, "{}\n", stmt)?;
                }
                Ok(())
            }
            Stmt::Break => write!(f, "break\n"),
            Stmt::While(condition, body) => {
                write!(f, "while {}:\n", condition)?;
                write!(f, "{}", body)
            }
            Stmt::Print(value) => write!(f, "print {}\n", value),
        }
    }
}

pub fn parse_stmt(tokens: &mut &[Token]) -> Stmt {
    let token = &tokens[0];
    match token.token_type {
        TokenType::Def => parse_def(tokens),
        TokenType::If => parse_if(tokens),
        TokenType::Return => parse_return(tokens),
        TokenType::While => parse_while(tokens),
        TokenType::Break => parse_break(tokens),
        TokenType::Print => parse_print(tokens),
        _ => parse_expr_stmt(tokens),
    }
}

pub fn skip_newline(tokens: &mut &[Token]) {
    let mut token = &tokens[0];
    while token.token_type == TokenType::NewLine {
        *tokens = &tokens[1..];
        token = &tokens[0];
    }
}

pub fn parse_def(tokens: &mut &[Token]) -> Stmt {
    *tokens = &tokens[1..];
    let name = parse_identifier(tokens);
    let mut params = Vec::new();
    let mut token = &tokens[0];
    if token.token_type == TokenType::LParen {
        *tokens = &tokens[1..];
        token = &tokens[0];
        while token.token_type != TokenType::RParen {
            let param = parse_expr(tokens);
            params.push(param);
            token = &tokens[0];
            if token.token_type == TokenType::Comma {
                *tokens = &tokens[1..];
            }
        }
        *tokens = &tokens[1..];
    }
    *tokens = &tokens[1..];
    skip_newline(tokens);

    let body = parse_block(tokens);

    Stmt::Def(name, params, Box::new(body))
}

pub fn parse_if(tokens: &mut &[Token]) -> Stmt {
    *tokens = &tokens[1..];
    let condition = parse_expr(tokens);
    *tokens = &tokens[1..];
    skip_newline(tokens);
    let body = parse_block(tokens);
    Stmt::If(condition, Box::new(body), None)
}

pub fn parse_while(tokens: &mut &[Token]) -> Stmt {
    *tokens = &tokens[1..];
    let condition = parse_expr(tokens);
    *tokens = &tokens[1..];
    skip_newline(tokens);
    let body = parse_block(tokens);
    Stmt::While(condition, Box::new(body))
}

pub fn parse_break(tokens: &mut &[Token]) -> Stmt {
    *tokens = &tokens[1..];
    Stmt::Break
}

pub fn parse_identifier(tokens: &mut &[Token]) -> Expr {
    let token = &tokens[0];

    if token.token_type != TokenType::Identifier {
        panic!("expecting identifier");
    }
    *tokens = &tokens[1..];
    Expr::Identifier(token.clone())
}

pub fn parse_block(tokens: &mut &[Token]) -> Stmt {
    // remove indent
    *tokens = &tokens[1..];
    let mut token = &tokens[0];
    let mut stmts = Vec::new();
    while token.token_type != TokenType::Dedent {
        let stmt = parse_stmt(tokens);
        stmts.push(stmt);
        skip_newline(tokens);
        token = &tokens[0];
    }
    *tokens = &tokens[1..];
    Stmt::Block(stmts)
}

pub fn parse_expr_stmt(tokens: &mut &[Token]) -> Stmt {
    let expr = parse_expr(tokens);
    // handle new_line
    *tokens = &tokens[1..];
    Stmt::Expr(expr)
}

pub fn parse_return(tokens: &mut &[Token]) -> Stmt {
    *tokens = &tokens[1..];
    let expr = parse_expr(tokens);
    Stmt::Return(expr)
}

pub fn parse_print(tokens: &mut &[Token]) -> Stmt {
    *tokens = &tokens[1..];
    let expr = parse_expr(tokens);
    Stmt::Print(expr)
}

pub fn parse_expr(tokens: &mut &[Token]) -> Expr {
    parse_assignment(tokens)
}

pub fn parse_assignment(tokens: &mut &[Token]) -> Expr {
    let name = parse_equality(tokens);
    let token = &tokens[0];
    if token.token_type == TokenType::Assign {
        *tokens = &tokens[1..];
        let value = parse_expr(tokens);
        Expr::Assign(Box::new(name), Box::new(value))
    } else {
        name
    }
}

pub fn parse_equality(tokens: &mut &[Token]) -> Expr {
    let mut expr = parse_comparison(tokens);
    loop {
        let token = &tokens[0];
        if token.token_type != TokenType::EqualEqual && token.token_type != TokenType::BangEqual {
            break;
        }
        *tokens = &tokens[1..];
        let right = parse_comparison(tokens);
        expr = Expr::BinaryOp(Box::new(expr), Box::new(right), token.clone());
    }
   
    expr
}

pub fn parse_comparison(tokens: &mut &[Token]) -> Expr {
    let mut expr = parse_term(tokens);
    loop {
        let token = &tokens[0];
        if token.token_type != TokenType::LessThan {
            break;
        }
        *tokens = &tokens[1..];
        let right = parse_term(tokens);
        expr = Expr::BinaryOp(Box::new(expr), Box::new(right), token.clone());
    }
    expr
}

pub fn parse_term(tokens: &mut &[Token]) -> Expr {
    let mut expr = parse_factor(tokens);
    loop {
        let token = &tokens[0];
        if token.token_type != TokenType::Plus && token.token_type != TokenType::Minus {
            break;
        }
        *tokens = &tokens[1..];
        let right = parse_factor(tokens);
        expr = Expr::BinaryOp(Box::new(expr), Box::new(right), token.clone());
    }
    expr
}

pub fn parse_factor(tokens: &mut &[Token]) -> Expr {
    let mut expr = parse_unary(tokens);
    loop {
        let token = &tokens[0];
        if token.token_type != TokenType::Mul && token.token_type != TokenType::Div {
            break;
        }
        *tokens = &tokens[1..];
        let right = parse_unary(tokens);
        expr = Expr::BinaryOp(Box::new(expr), Box::new(right), token.clone());
    }
    expr
}

pub fn parse_unary(tokens: &mut &[Token]) -> Expr {
    let token = &tokens[0];
    if token.token_type == TokenType::Minus {
        *tokens = &tokens[1..];
        let right = parse_unary(tokens);
        return Expr::BinaryOp(
            Box::new(Expr::IntLiteral(0)),
            Box::new(right),
            token.clone(),
        );
    }

    parse_call(tokens)
}

pub fn parse_call(tokens: &mut &[Token]) -> Expr {
    let mut expr = parse_primary(tokens);
    loop {
        let token = &tokens[0];
        if token.token_type != TokenType::LParen {
            break;
        }
        *tokens = &tokens[1..];
        let mut args = Vec::new();
        loop {
            let token = &tokens[0];
            if token.token_type == TokenType::RParen {
                break;
            }
            let arg = parse_expr(tokens);
            args.push(arg);
            let token = &tokens[0];
            if token.token_type == TokenType::RParen {
                break;
            }
            *tokens = &tokens[1..];
        }
        *tokens = &tokens[1..];
        expr = Expr::Call(Box::new(expr), args);
    }
    expr
}

pub fn parse_primary(tokens: &mut &[Token]) -> Expr {
    let token = &tokens[0];
    *tokens = &tokens[1..];
    match token.token_type {
        TokenType::Int => Expr::IntLiteral(token.value.parse().unwrap()),
        TokenType::Identifier => Expr::Identifier(token.clone()),
        TokenType::LParen => {
            let expr = parse_expr(tokens);
            let token = &tokens[0];
            if token.token_type != TokenType::RParen {
                panic!("Expected )");
            }
            *tokens = &tokens[1..];
            expr
        }
        _ => panic!("Unexpected token {:?}", token),
    }
}

pub fn parse_program(tokens: &mut &[Token]) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    loop {
        skip_newline(tokens);
        // println!("looping {:?}", tokens[0]);
        if tokens[0].token_type == TokenType::EndMarker {
            break;
        }
        let stmt = parse_stmt(tokens);
        stmts.push(stmt);
    }
    stmts
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let tokens = &mut &tokens[..];
    parse_program(tokens)
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{read_lines, tokenize};

    use super::*;

    #[test]
    fn test_parse_assign() {
        let source = read_lines("tests/var.py");
        let tokens = tokenize(&source);
        let mut tokens = &mut &tokens[..];
        let stmts = parse_program(&mut tokens);
        for stmt in stmts {
            println!("{}", stmt.pretty_print(0));
        }
    }

    #[test]
    fn test_parse_def() {
        let source = read_lines("tests/fib.py");
        let tokens = tokenize(&source);
        let mut tokens = &mut &tokens[..];
        let stmts = parse_program(&mut tokens);
        for stmt in stmts {
            println!("{}", stmt.pretty_print(0));
        }
    }

    #[test]
    fn test_parse_ambiguous() {
        let source = read_lines("tests/ambiguous.py");
        let tokens = tokenize(&source);
        let mut tokens = &mut &tokens[..];
        let stmts = parse_program(&mut tokens);
        for stmt in stmts {
            println!("{}", stmt.pretty_print(0));
        }
    }
    #[test]
    fn test_parse_empty_new_line() {
        let source = read_lines("tests/def.py");
        let tokens = tokenize(&source);
        let mut tokens = &mut &tokens[..];
        let stmts = parse_program(&mut tokens);
        for stmt in stmts {
            println!("{}", stmt.pretty_print(0));
        }
    }

    #[test]
    fn test_parse_while_stmt() {
        let source = read_lines("tests/while.py");
        let tokens = tokenize(&source);
        let mut tokens = &mut &tokens[..];
        let stmts = parse_program(&mut tokens);
        for stmt in stmts {
            // println!("{}", stmt.pretty_print(0));
            println!("{:?}", stmt);
        }
    }

    #[test]
    fn test_parse_print_stmt() {
        let source = read_lines("tests/print.py");
        let tokens = tokenize(&source);
        let mut tokens = &mut &tokens[..];
        let stmts = parse_program(&mut tokens);
        for stmt in stmts {
            println!("{}", stmt.pretty_print(0));
        }
    }

}
