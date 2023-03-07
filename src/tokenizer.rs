

use core::panic;

use phf::phf_map;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    And,
    As,
    Assert,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    False,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    None,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    True,
    Try,
    While,
    With,
    Yield,


    EndMarker,
    Name,
    Number,
    String,
    Newline,
    Indent,
    Dedent,
    Lpar,
    Rpar,
    Lsqb,
    Rsqb,
    Colon,
    Comma,
    Semi,
    Plus,
    Minus,
    Star,
    Slash,
    Vbar,
    Amper,
    Less,
    Greater,
    Equal,
    Dot,
    Percent,
    Lbrace,
    Rbrace,
    EqEqual,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Tilde,
    Circumflex,
    LeftShift,
    RightShift,
    DoubleStar,
    DoubleSlash,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AmperEqual,
    VbarEqual,
    CircumflexEqual,
    LeftShiftEqual,
    RightShiftEqual,
    DoubleStarEqual,
    DoubleSlashEqual,
    At,
    AtEqual,
    Rarrow,
    Ellipsis,
    ColonEqual,
    Comment
}


static KEYWORDS : phf::Map<&'static str, TokenType> = phf_map!(
    "and" => TokenType::And,
    "as" => TokenType::As,
    "assert" => TokenType::Assert,
    "break" => TokenType::Break,
    "class" => TokenType::Class,
    "continue" => TokenType::Continue,
    "def" => TokenType::Def,
    "del" => TokenType::Del,
    "elif" => TokenType::Elif,
    "else" => TokenType::Else,
    "except" => TokenType::Except,
    "False" => TokenType::False,
    "finally" => TokenType::Finally,
    "for" => TokenType::For,
    "from" => TokenType::From,
    "global" => TokenType::Global,
    "if" => TokenType::If,
    "import" => TokenType::Import,
    "in" => TokenType::In,
    "is" => TokenType::Is,
    "lambda" => TokenType::Lambda,
    "None" => TokenType::None,
    "nonlocal" => TokenType::Nonlocal,
    "not" => TokenType::Not,
    "or" => TokenType::Or,
    "pass" => TokenType::Pass,
    "raise" => TokenType::Raise,
    "return" => TokenType::Return,
    "True" => TokenType::True,
    "try" => TokenType::Try,
    "while" => TokenType::While,
    "with" => TokenType::With,
    "yield" => TokenType::Yield,
);

pub struct SourceRef<'source> {
    pub source: &'source str,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl<'source> SourceRef<'source> {
    pub fn new(source: &'source str, line: usize, start: usize, end: usize) -> SourceRef<'source> {
        SourceRef {
            source,
            line,
            start,
            end,
        }
    }

    pub fn value(&self) -> &str {
        &self.source[self.start..self.end]
    }
}


pub struct Token<'source> {
    pub token_type: TokenType,
    pub source_ref: SourceRef<'source>
}

impl<'source> Token<'source> {
    pub fn new(token_type: TokenType, start: usize, end: usize, line: usize, source: &str) -> Token {
        Token {
            token_type,
            source_ref: SourceRef::new(source, line, start, end)
        }
    }

    pub fn endmarker(start: usize, line: usize, source: &str) -> Token {
        Token {
            token_type: TokenType::EndMarker,
            source_ref: SourceRef::new(source, line, start, start)
        }
    }

    pub fn value(&self) -> &str {
        self.source_ref.value()
    }

    pub fn line(&self) -> usize {
        self.source_ref.line
    }

    pub fn start(&self) -> usize {
        self.source_ref.start
    }

    pub fn end(&self) -> usize {
        self.source_ref.end
    }

    pub fn length(&self) -> usize {
        self.end() - self.start()
    }
}


pub struct Tokenizer<'source> {
    pub source: &'source str,
    pub current: usize,
    pub start: usize,
    pub line: usize,
}

impl <'source> Tokenizer<'source> {
    pub fn new(source: &'source str) -> Tokenizer<'source> {
        Tokenizer {
            source,
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn next(&mut self) -> Token<'source> {
        if self.current >= self.source.len() {
            return Token::endmarker(self.current, self.line, self.source);
        }
        let ch = self.source[self.current..].chars().next().unwrap();
        match ch {
            ' ' | '\r' | '\t' => {
               self.advance();
               self.next()
            }
            // is character
            'a'..='z' | 'A'..='Z' | '_' => {
                return self.get_identifier();
            }

            // is number
            '0'..='9' => {
                return self.get_number();
            }
            _ => {
                panic!("next error")
            }
        }

    }

    pub fn get_identifier(&mut self) -> Token<'source> {
        let identifier_pattern = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
        if self.match_pattern(&identifier_pattern) {
            // get the matched string value
            let text = &self.source[self.current..];
            let matched_value = identifier_pattern.find(text).unwrap();
            let matched_value = &text[matched_value.start()..matched_value.end()];
            self.current += matched_value.len();
            let token_type = KEYWORDS.get(matched_value).unwrap_or(&TokenType::Name);
            return Token::new(*token_type, self.start, self.current, self.line, self.source);
        }
        panic!("get_identifier error")   
    }

    pub fn get_number(&mut self) -> Token<'source> {
        let number_pattern = Regex::new(r"^[0-9]+").unwrap();
        if self.match_pattern(&number_pattern) {
            // get the matched string value
            let text = &self.source[self.current..];
            let matched_value = number_pattern.find(text).unwrap();
            let matched_value = &text[matched_value.start()..matched_value.end()];
            self.current += matched_value.len();
            return Token::new(TokenType::Number, self.start, self.current, self.line, self.source);
        }
        panic!("get_number error")
    }

    // pub fn get_operator(&mut self) -> Token<'source> {
    //     let operator_pattern = Regex::new(r"^[+*/-]").unwrap();
    //     if self.match_pattern(&operator_pattern) {
    //         // get the matched string value
    //         let text = &self.source[self.current..];
    //         let matched_value = operator_pattern.find(text).unwrap();
    //         let matched_value = &text[matched_value.start()..matched_value.end()];
    //         self.current += matched_value.len();
    //         return Token::new(TokenType::Operator, self.start, self.current, self.line, self.source);
    //     }
    //     panic!("get_operator error")
    // }
    // check if the string start with the given pattern
    pub fn match_pattern(&mut self, pattern: &Regex) -> bool {
        let source = &self.source[self.current..];
        pattern.is_match(source)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let ch = self.source[self.current..].chars().next().unwrap();
        self.current += ch.len_utf8();
        ch
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_name() {
        let source = "name def";
        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();
        assert_eq!(token.token_type, TokenType::Name);
        let token = tokenizer.next();
        assert_eq!(token.token_type, TokenType::Def);
        let token = tokenizer.next();
        assert_eq!(token.token_type, TokenType::EndMarker);
    }
}