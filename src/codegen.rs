use regex::internal::Inst;

use crate::{
    intruction::Instruction,
    parser::{Expr, Stmt},
    token::TokenType,
};

pub fn codegen_expr(expr: &Expr) -> Vec<Instruction> {
    match expr {
        Expr::IntLiteral(i) => vec![Instruction::LoadConst(*i)],
        Expr::Identifier(i) => vec![Instruction::LoadName(i.value.clone())],
        Expr::BinaryOp(left, right, op) => {
            let left = codegen_expr(left);
            let right = codegen_expr(right);
            let mut instructions = Vec::new();
            instructions.extend(left);
            instructions.extend(right);
            instructions.push(Instruction::BinaryOp(op.value.clone()));
            instructions
        }
        Expr::Assign(name, value) => {
            let value = codegen_expr(value);
            let mut instructions = Vec::new();
            instructions.extend(value);
            match name.as_ref() {
                Expr::Identifier(name) => {
                    instructions.push(Instruction::StoreName(name.value.clone()));
                }
                _ => panic!("Invalid assignment"),
            };
            instructions
        }
        _ => panic!("Unknown expression"),
    }
}

// pub fn codegen_stmt(stmt: &Stmt) -> Vec<Instruction> {
//     match stmt {
//         Stmt::Expr(expr) => codegen_expr(expr),

//         Stmt::Return(expr) => {
//             let expr = codegen_expr(expr);
//             let mut instructions = Vec::new();
//             instructions.extend(expr);
//             instructions.push(Instruction::Return);
//             instructions
//         }
//         _ => panic!("Unknown statement"),
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    use crate::{
        parser::parse,
        token,
        tokenizer::{read_lines, tokenize},
    };

    #[test]

    fn test_binary_expr() {
        let expr = Expr::BinaryOp(
            Box::new(Expr::IntLiteral(1)),
            Box::new(Expr::IntLiteral(2)),
            token::Token {
                token_type: TokenType::Plus,
                value: "+".to_string(),
            },
        );
        let instructions = codegen_expr(&expr);
        assert_eq!(
            instructions,
            vec![
                Instruction::LoadConst(1),
                Instruction::LoadConst(2),
                Instruction::BinaryOp("+".to_string())
            ]
        );
    }
    #[test]
    fn test_assign() {
        let expr = Expr::Assign(
            Box::new(Expr::Identifier(token::Token {
                token_type: TokenType::Identifier,
                value: "a".to_string(),
            })),
            Box::new(Expr::IntLiteral(1)),
        );
        let instructions = codegen_expr(&expr);
        assert_eq!(
            instructions,
            vec![
                Instruction::LoadConst(1),
                Instruction::StoreName("a".to_string())
            ]
        );
    }
}
