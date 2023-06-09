use crate::{
    expressions::{
        BinaryExpr, Expr, Expr::*, GroupingExpr, LiteralExpr, LiteralExpr::*, UnaryExpr,
    },
    types::TokenType,
};

/// Takes the root of the AST and evaluates it down to a result.
pub fn interpret(input: Expr) -> Expr {
    input.evaluated()
}

/// Errors that happen at runtime: Ex at evaluating an Expression, trying to divide by 0;
#[derive(Debug, Clone, PartialEq)]
pub enum RunErr {
    NotImplementedUnaryExpr,
    NotImplementedBinaryExpr,
    FailedAddition,
    FailedEqual,
    FailedDivision,
    FailedDivisionByZero,
    FailedMultiplication,
    FailedComparison,
    FailedSubtraction,
}

// interface to evaluate our expressions. (1+3 resolves to 4) => we keep 4 "and throw the rest away"
pub trait Evaluates {
    fn evaluated(&self) -> Expr;
}

impl Evaluates for Expr {
    fn evaluated(&self) -> Self {
        match self {
            RuntimeErr(e) => RuntimeErr(e.clone()),
            ErrorExpr => ErrorExpr,
            Literal(expr) => expr.evaluated(),
            Grouping(expr) => expr.evaluated(),
            Unary(expr) => expr.evaluated(),
            Binary(expr) => expr.evaluated(),
        }
    }
}

impl Evaluates for LiteralExpr {
    fn evaluated(&self) -> Expr {
        return Literal(self.clone());
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
            (TokenType::Minus, Literal(Number(nr))) => Literal(Number(-nr)),
            (TokenType::Exclamation, Literal(Boolean(istrue))) => Literal(Boolean(!istrue)),
            // !nil = true :
            (TokenType::Exclamation, Literal(Nil)) => Literal(Boolean(true)),
            _ => RuntimeErr(RunErr::NotImplementedUnaryExpr),
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
            _ => RuntimeErr(RunErr::NotImplementedBinaryExpr),
        }
    }
}

// helper function to evaluate BinaryExpr:
fn subtraction(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (Literal(Number(l)), TokenType::Minus, Literal(Number(r))) => Literal(Number(l - r)),
        _ => RuntimeErr(RunErr::FailedSubtraction),
    }
}

// helper function to evaluate BinaryExpr:
fn multiplication(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (Literal(Number(l)), TokenType::Star, Literal(Number(r))) => Literal(Number(l * r)),
        _ => RuntimeErr(RunErr::FailedMultiplication),
    }
}

// helper function to evaluate BinaryExpr:
fn division(left: Expr, token: TokenType, right: Expr) -> Expr {
    // explicit checking for division by 0 errors:
    if let Literal(Number(nr)) = right {
        if nr == 0.0 || nr == -0.0 {
            return RuntimeErr(RunErr::FailedDivisionByZero);
        }
    }

    match (left, token, right) {
        (Literal(Number(l)), TokenType::Slash, Literal(Number(r))) => Literal(Number(l / r)), // TODO we might have to handle divide by 0
        _ => RuntimeErr(RunErr::FailedDivision),
    }
}

// helper function to evaluate BinaryExpr:
fn addition(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        // addition
        (Literal(Number(l)), TokenType::Plus, Literal(Number(r))) => Literal(Number(l + r)),
        // string concatinations:
        (Literal(String(l)), TokenType::Plus, Literal(Number(r))) => {
            Literal(String(l + &r.to_string()))
        }
        (Literal(String(l)), TokenType::Plus, Literal(Boolean(r))) => {
            Literal(String(l + &r.to_string()))
        }
        (Literal(String(l)), TokenType::Plus, Literal(Nil)) => Literal(String(l + "Nil")),
        (Literal(String(l)), TokenType::Plus, Literal(String(r))) => Literal(String(l + &r)),
        _ => RuntimeErr(RunErr::FailedAddition),
    }
}

// helper function to evaluate BinaryExpr:
fn comparison(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (Literal(Number(l)), TokenType::Less, Literal(Number(r))) => Literal(Boolean(l < r)),
        (Literal(Number(l)), TokenType::LessEqual, Literal(Number(r))) => Literal(Boolean(l <= r)),
        (Literal(Number(l)), TokenType::Greater, Literal(Number(r))) => Literal(Boolean(l > r)),
        (Literal(Number(l)), TokenType::GreaterEqual, Literal(Number(r))) => {
            Literal(Boolean(l >= r))
        }
        _ => RuntimeErr(RunErr::FailedComparison),
    }
}

// helper function to evaluate BinaryExpr:
fn is_equal(left: Expr, token: TokenType, right: Expr) -> Expr {
    match (left, token, right) {
        (l, TokenType::ExclamationEqual, r) => Literal(Boolean(l != r)),
        (l, TokenType::EqualEqual, r) => Literal(Boolean(l == r)),
        _ => RuntimeErr(RunErr::FailedEqual),
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use crate::{lexer, parser::AST};
    use RunErr::*;


    use super::*;
    // some quick integration testing:
    fn test(input: &str, expected: Expr){
        let s = lexer::new_scanner(input);
        let (tokens, lexer_errs) = s.results();
        let ast = AST::new(tokens);
        let result = interpret(ast.root);
        
        assert_eq!(result, expected);
        assert!(lexer_errs.len() == 0);
        assert!(ast.errors.len() == 0);
    }

    #[test]
    fn equal() {
        // AST: <true == false>     =>   true
        test("true == false;", Literal(Boolean(false)));
        test("true != true;", Literal(Boolean(false)));
        test("10 == 10;", Literal(Boolean(true)));
        test("\"hello\" != \"hello\";", Literal(Boolean(false)));
        test("nil == nil;", Literal(Boolean(true)));
    }

    #[test]
    fn comparison() {
        test("1 < 2", Literal(Boolean(true)));
        test("10<=10", Literal(Boolean(true)));
        test("10>10", Literal(Boolean(false)));
        test("10>=10", Literal(Boolean(true)));
        test("true > false", RuntimeErr(FailedComparison));
        test("\"hello\" <= \"hello\";", RuntimeErr(FailedComparison));
    }

    #[test]
    fn addition_subtraction() {
        test("1.1+2", Literal(Number(3.1)));
        test("1.2-2", Literal(Number(-0.8)));
        test("\"hello\" + \" bye\";", Literal(String("hello bye".to_string())));
        test("\"hello_\" + nil", Literal(String("hello_Nil".to_string())));
        test("\"hello_\" + 3", Literal(String("hello_3".to_string())));
        test("\"hello_\" + true", Literal(String("hello_true".to_string())));
        test("\"hello_\" + false", Literal(String("hello_false".to_string())));
        test("true+false", RuntimeErr(FailedAddition));
    }

    #[test]
    fn division() {
        test("1/2", Literal(Number(0.5)));
        test("-2/0.5", Literal(Number(-4.0)));
        test("1/0", RuntimeErr(FailedDivisionByZero));
        test("10 / nil", RuntimeErr(FailedDivision));
        test("true / 2", RuntimeErr(FailedDivision));
    }

    #[test]
    fn multiplication() {
        test("1.1*2", Literal(Number(2.2)));
        test("-1.2*0.2", Literal(Number(-0.24)));
        test("-1.2*nil", RuntimeErr(FailedMultiplication));
    }
}
