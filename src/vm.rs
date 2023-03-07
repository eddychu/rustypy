// use crate::{environment::Environment, intruction::Instruction, value::Value};

// pub struct Vm {
//     pub codes: Vec<Instruction>,
//     pub stack: Vec<Value>,
//     pub env: Environment,
// }

// impl Vm {
//     pub fn new() -> Self {
//         Self {
//             codes: Vec::new(),
//             stack: Vec::new(),
//             env: Environment::new(),
//         }
//     }

//     pub fn run(&mut self) -> Value {
//         let mut pc = 0;
//         loop {
//             if pc >= self.codes.len() {
//                 break;
//             }

//             let instruction = &self.codes[pc];
//             match instruction {
//                 Instruction::LoadConst(value) => {
//                     self.stack.push(Value::Int(*value));
//                 }
//                 Instruction::LoadName(name) => {
//                     let value = self.env.get(name, 0).unwrap();
//                     self.stack.push(value);
//                 }
//                 Instruction::StoreName(name) => {
//                     let value = self.stack.pop().unwrap();
//                     self.env.set(name, value, 0);
//                 }
//                 Instruction::BinaryOp(op) => {
//                     if let Value::Int(right) = self.stack.pop().unwrap() {
//                         if let Value::Int(left) = self.stack.pop().unwrap() {
//                             let result = match op.as_str() {
//                                 "+" => left + right,
//                                 "-" => left - right,
//                                 "*" => left * right,
//                                 "/" => left / right,
//                                 "%" => left % right,
//                                 _ => panic!("Unknown operator"),
//                             };
//                             self.stack.push(Value::Int(result));
//                         }
//                     } else {
//                         panic!("Invalid operand");
//                     }
//                 }
//             }
//             pc += 1;
//         }
//         self.stack.pop().unwrap()
//     }
// }

// #[cfg(test)]

// mod tests {

//     use super::*;

//     use crate::{parser::parse, token::TokenType};

//     #[test]
//     fn test_vm() {
//         let mut vm = Vm::new();
//         let codes = vec![
//             Instruction::LoadConst(1),
//             Instruction::LoadConst(2),
//             Instruction::BinaryOp("+".to_string()),
//             Instruction::LoadConst(3),
//             Instruction::BinaryOp("+".to_string()),
//             Instruction::LoadConst(4),
//             Instruction::BinaryOp("+".to_string()),
//         ];
//         vm.codes = codes;
//         let result = vm.run();
//         assert_eq!(result, Value::Int(10));
//         // assert_eq!(result, Value::Int(7));
//     }
// }
