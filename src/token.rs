use std::fmt;

// The Tokens that get created at the lexer then passed over to the parser
#[derive(Debug)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
    // char_nr: usize,      // to add error messages with char-nr for each line
}
impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "typ: <{}> lexeme: <{}> line:<{}>",
            self.typ, self.lexeme, self.line
        )
    }
}

// all Types of Shapes the Token can take in our programming language
#[rustfmt::skip]  // ignore autoformater on this enum
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // single-character tokens
    OpenParen, CloseParen, OpenBrace, CloseBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // 1-2 character tokens
    Exclamation, ExclamationEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier,
    String(String),
    Number(f64),

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
