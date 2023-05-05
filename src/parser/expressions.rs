use crate::token::TokenType;

// interface for all Expressions. They are the building blocks of our AST
// We expose those to our backend-parser AND frontend-lexxer

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// experimental, to avoid having to bubble up Errors and use Result<Expr> everywhere
    /// TODO: check if this holds up
    ErrorExpr,
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
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

            //Failback to Debug-Printing for unimplemented ones:
            _ => write!(f, "{:?}", self),
        }
    }
}
