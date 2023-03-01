// use std::collections::HashMap;

// use crate::interpreter::Value;

// #[derive(Debug, Clone, PartialEq)]
// pub struct Environment {
//     pub variables: HashMap<String, Value>,
//     pub enclosing: Option<Box<Environment>>,
// }

// impl Environment {
//     pub fn new() -> Self {
//         Self {
//             variables: HashMap::new(),
//             enclosing: None,
//         }
//     }

//     pub fn new_with_enclosing(enclosing: Environment) -> Self {
//         Self {
//             variables: HashMap::new(),
//             enclosing: Some(Box::new(enclosing)),
//         }
//     }

//     pub fn get(&self, name: &str) -> Option<&Value> {
//         self.variables.get(name)
//     }

//     pub fn define(&mut self, name: String, value: Value) {
//         self.variables.insert(name, value);
//     }

//     pub fn has(&self, name: &str) -> bool {
//         self.variables.contains_key(name)
//             || match &self.enclosing {
//                 Some(enclosing) => enclosing.has(name),
//                 None => false,
//             }
//     }

//     pub fn assign(&mut self, name: String, value: Value) {
//         if !self.variables.contains_key(&name) {
//             self.variables.insert(name, value);
//         }
//     }
// }
