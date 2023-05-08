use crate::{interpreter::RunErr, types::TokenType};

// Collection of all Expressions. They are the building blocks of our AST
// We expose those to our backend-interpreter AND middle-parser

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    /// When the Parser fails it creates a Stand in ErrorToken to continue parsing the rest
    ErrorExpr,
    /// Run time Errors that happen in the Interpreter
    RuntimeErr(RunErr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub token: TokenType,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub token: TokenType,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupingExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExpr {
    Boolean(bool),
    Nil,
    String(String),
    Number(f64),
}

// Display Trait used for pretty-printing the ast tree:
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::ErrorExpr => f.write_str("ErrorExpr"),
            Expr::Literal(LiteralExpr::Boolean(b)) => b.fmt(f),
            Expr::Literal(LiteralExpr::Nil) => f.write_str("Nil"),
            Expr::Literal(LiteralExpr::String(s)) => s.fmt(f),
            Expr::Literal(LiteralExpr::Number(n)) => n.fmt(f),

            Expr::Binary(BinaryExpr { left, token, right }) => {
                f.write_fmt(format_args!("<{left} {token} {right}>"))
            }
            Expr::Unary(UnaryExpr { token, right }) => {
                f.write_fmt(format_args!("<{token} {right}>"))
            }
            Expr::Grouping(GroupingExpr { expr }) => f.write_fmt(format_args!("({expr})")),
            Expr::RuntimeErr(e) => write!(f, "RuntimeErr({:?})", e),
            //_ => write!(f, "{:?}", self),             //Failback to Debug-Printing for unimplemented ones:
        }
    }
}

