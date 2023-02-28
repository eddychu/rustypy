#[derive(Debug, PartialEq)]
pub enum TokenType {
    Def,
    If,
    Return,
    Identifier,
    Int,
    Plus,
    Minus,
    Mul,
    Div,
    Colon,
    LessThan,
    // GreaterThan,
    Indent,
    Dedent,

    LParen,
    RParen,

    NewLine,
    EndMarker,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
