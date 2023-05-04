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
    pub left: Option<Box<Expr>>, // TODO: can we do without the Option? https://github.com/LukeMathWalker/jlox-rs/blob/main/src/parser/ast.rs
    pub token: TokenType, // TODO: would be cleaner to have own enum only allowing supported Tokens!
    pub right: Option<Box<Expr>>,
}

pub struct UnaryExpr {
    pub token: TokenType, // TODO: would be cleaner to have own enum only allowing supported Tokens!
    pub right: Option<Box<Expr>>,
}

pub struct GroupingExpr {
    pub expr: Option<Box<Expr>>,
}

pub struct LiteralExpr {
    //todo
}
