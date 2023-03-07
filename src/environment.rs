// use std::{collections::HashMap, borrow::Borrow};

// use crate::value::Value;

// #[derive(Debug, Clone)]
// pub struct Environment {
//     pub envs: Vec<Frame>,
// }

// // pretty print the environment

// impl std::fmt::Display for Environment {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut current_frame = self.envs.len() - 1;
//         let mut indent = 0;
//         loop {
//             indent += 1;
//             let leading = " ".repeat(indent * 4);
//             let frame = &self.envs[current_frame];
//             writeln!(f, "{}Frame {}", leading, frame.id)?;
//             for (key, value) in &frame.variables {
//                 writeln!(f, "{}{}: {:?}", leading, key, value)?;
//             }
//             if let Some(parent) = frame.parent {
//                 current_frame = parent;
//             } else {
//                 break;
//             }
//         }
//         Ok(())
//     }
// }

// impl Environment {
//     pub fn new() -> Self {
//         Self {
//             envs: vec![Frame::new(0)],
//         }
//     }

//     pub fn allocate_new_frame(&mut self) -> usize {
//         let new_id = self.envs.len() - 1;
//         self.envs.push(Frame::new(new_id));
//         new_id
//     }

//     pub fn get(&self, name: &str, frame_index: usize) -> Option<Value> {
//         let mut current_frame = frame_index;
//         loop {
//             let frame = &self.envs[current_frame];
//             if let Some(value) = frame.variables.get(name) {
//                 return Some(value.clone());
//             }
//             if let Some(parent) = frame.parent {
//                 current_frame = parent;
//             } else {
//                 return None;
//             }
//         }
//     }

//     pub fn set(&mut self, name: &str, value: Value, frame_index: usize) {
//         let mut current_frame = frame_index;
//         loop {
//             let frame = &mut self.envs[current_frame];
//             if frame.variables.contains_key(name) {
//                 frame.variables.insert(name.to_string(), value);
//                 return;
//             }
//             if let Some(parent) = frame.parent {
//                 current_frame = parent;
//             } else {
//                 break;
//             }
//         }
//         self.envs[frame_index].variables.insert(name.to_string(), value);
//     }

   
// }

// #[derive(Debug, Clone)]
// pub struct Frame {
//     pub variables: HashMap<String, Value>,
//     pub parent: Option<usize>,
//     pub id: usize,
// }
// impl Frame {
//     pub fn new(id: usize) -> Self {
//         Self {
//             variables: HashMap::new(),
//             parent: None,
//             id,
//         }
//     }
// }
