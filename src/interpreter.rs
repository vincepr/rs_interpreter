use std::collections::HashMap;

use crate::{
    expressions::{
        BinaryExpr, Expr, Expr::*, GroupingExpr, LiteralExpr, LiteralExpr::*, UnaryExpr, VarAssignExpr, VarReadExpr,
    },
    types::TokenType, statements::{Statement}, environment::Environment,
};

/// Takes the root of the AST and evaluates it down to a result.
pub fn interpret(inputs: Vec<Statement>) {
    // envirnoment that holds reference to all variable-names-> values mapped:
    //let mut globalScope = crate::environment::Environment::new(None);
    let mut global_scope = crate::environment::Environment{
        enclosing: None ,
        values: HashMap::new(),
    };

    for statement in inputs{
        exececute(&mut global_scope, statement);
    }
}

/*
        Statements Execute, always end with a ;
*/

fn exececute(scope: &mut Environment, statement: Statement) {
    // TODO: Here should be a good place to check for errors? check if we get an error then print that out or smth
    statement.execute(scope);
}

/// gets called from Statements-'visitorpattern'
pub fn execute_block<'a>(parent_scope:&'a mut Environment<'a>, statements: Vec<Statement>) {
    // create the new Scope:
    //let mut  localScope = Environment::new(Some(parentScope));
    let mut local_scope = crate::environment::Environment{
        enclosing: Some(parent_scope) ,
        values: HashMap::new(),
    };
    for statement in statements {
        exececute(&mut local_scope, statement);
    }
}

/*
        Expressions Evaluate, something that evaluates to a value -> x+1 or true==nil
*/

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
trait Evaluates {
    fn evaluated(&self, env: &mut Environment) -> Expr;
}

impl Expr {
    /// maps the visitor-patern like implementations of how different expressions evaluate:
    pub fn evaluated(&self, env: &mut Environment) -> Self {
        match self {
            RuntimeErr(e) => RuntimeErr(e.clone()),
            ErrorExpr => ErrorExpr,
            Literal(expr) => expr.evaluated(env),
            Grouping(expr) => expr.evaluated(env),
            Unary(expr) => expr.evaluated(env),
            Binary(expr) => expr.evaluated(env),

            VarAssign(expr) => expr.eval_with_env(env),
            VarRead(expr) => expr.eval_with_env(env),
        }
    }
}

impl VarAssignExpr {
    fn eval_with_env(&self, env: &mut Environment) -> Expr {
        let new_val = self.value.evaluated(env);
        env.assign(self.name.clone(), *self.value.clone());
        return new_val;
    }
}

impl VarReadExpr {
    fn eval_with_env(&self, env: &mut Environment) -> Expr {
        env.get_value(self.name.clone()).unwrap()       // TODO: we need to properly handle this err
    }
}

impl Evaluates for LiteralExpr {
    fn evaluated(&self, env: &mut Environment) -> Expr {
        return Literal(self.clone());
    }
}

impl Evaluates for GroupingExpr {
    fn evaluated(&self, env: &mut Environment) -> Expr {
        return self.expr.evaluated(env);
    }
}

impl Evaluates for UnaryExpr {
    fn evaluated(&self, env: &mut Environment) -> Expr {
        let right = (*self.right).evaluated(env);

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
    fn evaluated(&self, env: &mut Environment) -> Expr {
        let left = (*self.left).evaluated(env);
        let right = (*self.right).evaluated(env);

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
        //
        let mut global_scope = crate::environment::Environment{
            enclosing: None ,
            values: HashMap::new(),
        };
        //
        let s = lexer::new_scanner(input);
        let (tokens, lexer_errs) = s.results();
        let ast = AST::new(tokens);
        let statements = ast.root;
        for s in statements{
            if let Statement::ExprSt(expr) = s{
                let res = expr.evaluated(&mut global_scope);
                assert_eq!(res, expected);
            }else { panic!("expected a Expression that evaluates!")}
            assert!(lexer_errs.len() == 0);
            assert!(ast.errors.len() == 0);
        }



        
        
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
