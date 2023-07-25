use std::fmt;

/*
    Basic collection types that get passed arround between module-borders.
*/

// Possible Errors get defined by this
#[derive(Debug, Clone)]
pub enum Err {
    // WhatFailed (Error-message, line-of-error)
    //      TODO: could change to point to character/byte of error
    //      -> re-evaluate to line && character in line
    Parser(String, usize),
    Lexer(String, usize),
    Interpreter(String, usize),
}
impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::Lexer(message, line) => {
                f.write_fmt(format_args!("Lexer-ERROR in line: {line} : {message}!"))
            }
            Err::Parser(message, line) => {
                f.write_fmt(format_args!("ParserERROR in line: {line} : {message}!"))
            }
            Err::Interpreter(message, line) => f.write_fmt(format_args!(
                "Interpreter-ERROR in line: {line} : {message}!"
            )),
        }
    }
}

// The Tokens that get created at the lexer then passed over to the parser
#[derive(Debug, PartialEq)]
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
            "{}<-token | lexeme->{} | line:{}",
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

// We need Display-Trait for proper error messages or AST-Tree logging...
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenType::OpenParen => f.write_str("("),
            TokenType::CloseParen => f.write_str(")"),
            TokenType::OpenBrace => f.write_str("{"),
            TokenType::CloseBrace => f.write_str("}"),
            TokenType::Comma => f.write_str(","),
            TokenType::Dot => f.write_str("."),
            TokenType::Minus => f.write_str("-"),
            TokenType::Plus => f.write_str("+"),
            TokenType::Semicolon => f.write_str(";"),
            TokenType::Slash => f.write_str("/"),
            TokenType::Star => f.write_str("*"),
            TokenType::Exclamation => f.write_str("!"),
            TokenType::ExclamationEqual => f.write_str("!="),
            TokenType::Equal => f.write_str("="),
            TokenType::EqualEqual => f.write_str("=="),
            TokenType::Greater => f.write_str(">"),
            TokenType::GreaterEqual => f.write_str(">="),
            TokenType::Less => f.write_str("<"),
            TokenType::LessEqual => f.write_str("<="),

            TokenType::Identifier => {
                f.write_str("TODO: Display for IDENTIFIER in types.TokenType.Display")
            }
            TokenType::String(s) => f.write_str(s),
            TokenType::Number(n) => n.fmt(f),

            // just using default {:?} for the following:
            // TokenType::And => f.write_str("("),
            // TokenType::Class => f.write_str(")"),
            // TokenType::Else => f.write_str("("),
            // TokenType::False => f.write_str(")"),
            // TokenType::Fun => f.write_str("("),
            // TokenType::For => f.write_str(")"),
            // TokenType::If => f.write_str("("),
            // TokenType::Nil => f.write_str(")"),
            // TokenType::Or => f.write_str("("),
            // TokenType::Print => f.write_str(")"),
            // TokenType::Return => f.write_str("("),
            // TokenType::Super => f.write_str(")"),
            // TokenType::This => f.write_str("("),
            // TokenType::True => f.write_str(")"),
            // TokenType::Var => f.write_str("("),
            // TokenType::While => f.write_str(")"),
            // TokenType::EOF => f.write_str("("),
            _ => write!(f, "{:?}", self),
        }
    }
}
