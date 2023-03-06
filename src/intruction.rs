use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Cache,
    PopTop,
    PushNull,
    InterpreterExit,

    EndFor = 4,
    Nop = 9,
    UnaryNegative = 11,
    UnaryNot = 12,
    UnaryInvert = 15,

    BinarySubscr = 25,
    BinarySlice = 26,
    StoreSlice = 27,

    GetLen = 30,
    MatchMapping = 31,
    MatchSequence = 32,
    MatchKeys = 33,

    ReturnValue = 83,
    StoreName = 90,
    LoadConst = 100,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    LoadConst(i32),
    StoreName(String),
    LoadName(String),
    BinaryOp(String),
}
