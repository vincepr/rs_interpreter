use crate::{expressions::*, types::TokenType};



pub fn interpret(input: Expr) -> Expr{
    input.evaluated()
}

// interface to evaluate our expressions. (1+3 resolves to 4) => we keep 4 "and throw the rest away"
pub trait Evaluates {
    fn evaluated(&self) -> Expr;
}

impl Evaluates for Expr {
    fn evaluated(&self) -> Self {
        match self{
            Expr::ErrorExpr => Expr::ErrorExpr,
            Expr::Literal(expr) => expr.evaluated(),
            Expr::Grouping(expr) => expr.evaluated(),
            Expr::Unary(expr) => expr.evaluated(),
            Expr::Binary(expr) => expr.evaluated(),
        }
    }
}

impl Evaluates for LiteralExpr {
    fn evaluated(&self) -> Expr {
        return Expr::Literal(self.clone()); // already handled by (impl Evaluates for Expr) but keeping it in for completeness
    }
}

impl Evaluates for GroupingExpr {
    fn evaluated(&self) -> Expr {
        return self.expr.evaluated();
    }
}

impl Evaluates for UnaryExpr {
    fn evaluated(&self) -> Expr {
        let right = (*self.right).evaluated();

        match (self.token.clone(), right) {
            (TokenType::Minus, Expr::Literal(LiteralExpr::Number(nr))) => {
                Expr::Literal(LiteralExpr::Number(-nr))
            }
            (TokenType::Exclamation, Expr::Literal(LiteralExpr::Boolean(istrue))) => {
                Expr::Literal(LiteralExpr::Boolean(!istrue))
            }
            // !nil = true :
            (TokenType::Exclamation, Expr::Literal(LiteralExpr::Nil)) => {
                Expr::Literal(LiteralExpr::Boolean(true))
            }
            _ => Expr::ErrorExpr, // TODO: Make sure this Error is handled upstream. Like if -"string" etc.
        }
    }
}

impl Evaluates for BinaryExpr {
    fn evaluated(&self) -> Expr {
        let left = (*self.left).evaluated();
        let right = (*self.right).evaluated();

        match (left, self.token.clone(), right) {
            (left, TokenType::Minus, right) => subtraction(left, TokenType::Minus, right),
            (left, TokenType::Slash, right) => division(left, TokenType::Slash, right),
            (left, TokenType::Star, right) => multiplication(left, TokenType::Star, right),
            (left, TokenType::Plus, right) => addition(left, TokenType::Plus, right),
            (
                left,
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual,
                right,
            ) => comparison(left, self.token.clone(), right),
            (left, TokenType::ExclamationEqual | TokenType::EqualEqual, right) => {
                is_equal(left, self.token.clone(), right)
            }

            _ => Expr::ErrorExpr,
        }
    }
}

// helper function to evaluate BinaryExpr:
fn subtraction(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::Minus,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Number(l - r)),
        _ => Expr::ErrorExpr,
    }
}

// helper function to evaluate BinaryExpr:
fn division(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::Slash,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Number(l / r)), // TODO we might have to handle divide by 0
        _ => Expr::ErrorExpr,
    }
}

// helper function to evaluate BinaryExpr:
fn multiplication(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::Star,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Number(l * r)),
        _ => Expr::ErrorExpr,
    }
}

// helper function to evaluate BinaryExpr:
fn addition(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        // addition
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::Plus,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Number(l + r)),
        // string concatinations:
        (
            Expr::Literal(LiteralExpr::String(l)),
            TokenType::Plus,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::String(l + &r.to_string())),

        (
            Expr::Literal(LiteralExpr::String(l)),
            TokenType::Plus,
            Expr::Literal(LiteralExpr::Boolean(r)),
        ) => Expr::Literal(LiteralExpr::String(l + &r.to_string())),
        (
            Expr::Literal(LiteralExpr::String(l)),
            TokenType::Plus,
            Expr::Literal(LiteralExpr::Nil),
        ) => Expr::Literal(LiteralExpr::String(l + "Nil")),
        (
            Expr::Literal(LiteralExpr::String(l)),
            TokenType::Plus,
            Expr::Literal(LiteralExpr::String(r)),
        ) => Expr::Literal(LiteralExpr::String(l + &r)),
        _ => Expr::ErrorExpr,
    }
}

// helper function to evaluate BinaryExpr:
fn comparison(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::Less,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Boolean(l < r)),
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::LessEqual,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Boolean(l <= r)),
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::Greater,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Boolean(l > r)),
        (
            Expr::Literal(LiteralExpr::Number(l)),
            TokenType::GreaterEqual,
            Expr::Literal(LiteralExpr::Number(r)),
        ) => Expr::Literal(LiteralExpr::Boolean(l >= r)),
        _ => Expr::ErrorExpr,
    }
}

// helper function to evaluate BinaryExpr:
fn is_equal(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (l, TokenType::ExclamationEqual, r) => Expr::Literal(LiteralExpr::Boolean(l != r)),
        (l, TokenType::EqualEqual, r) => Expr::Literal(LiteralExpr::Boolean(l == r)),
        _ => Expr::ErrorExpr,
    }
}