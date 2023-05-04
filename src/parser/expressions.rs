use crate::token::TokenType;

// our interface for all Expressions.
// We expose those to our backend-parser AND frontend-lexxer
// so lets try to keep them clean

pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
}

pub struct BinaryExpr {
    pub left: Box<Expr>, // TODO: can we do without the Option? https://github.com/LukeMathWalker/jlox-rs/blob/main/src/parser/ast.rs
    pub token: TokenType, // TODO: would be cleaner to have own enum only allowing supported Tokens!
    pub right: Box<Expr>,
}

pub struct UnaryExpr {
    pub token: TokenType, // TODO: would be cleaner to have own enum only allowing supported Tokens!
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expr: Box<Expr>,
}

pub enum LiteralExpr {
    Boolean(bool),
    Nil,
    String(String),
    Number(f64),
}
