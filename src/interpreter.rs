use std::rc::Rc;

use crate::{
    environment::Environment,
    expressions::{
        BinaryExpr, Expr, Expr::*, FnCallExpr, Function, GroupingExpr, LogicalExpr, UnaryExpr,
        Value, Value::*, VarAssignExpr, VarReadExpr,
    },
    statements::Statement,
    types::{Err, TokenType},
};

/// Takes the root of the AST and evaluates it down to a result.
pub fn interpret(inputs: Vec<Result<Statement, Err>>) {
    // envirnoment that holds reference to all variable-names-> values mapped:
    let global_scope: Rc<Environment> = Rc::new(Environment::new(None));

    for statement in inputs {
        exececute(global_scope.clone(), statement);
    }
}

/*
        To make native functions ex 'time()' accessible we inject them into the global_scope:
*/
fn build_global_scope() -> Rc<Environment> {
    let global_scope: Rc<Environment> = Rc::new(Environment::new(None));
    // next we inject our custom functions, so they become available in global scope:
    let clock = Expr::Literal(Value::Callable(Rc::new(Function::Native {
        arity: 0,
        func: clock_native,
    })));
    global_scope.define("clock".into(), clock);
    return global_scope;
}

fn clock_native() -> Result<Value, Err> {
    Ok(Value::Number(get_epoch_ms()))
}
fn get_epoch_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

/*
        Statements Execute, always end with a ;
*/

fn exececute(scope: Rc<Environment>, statement: Result<Statement, Err>) {
    match statement {
        Ok(st) => {
            // after trying execution we check if we hit an runtime error, if so we print then abort
            if let Err(e) = st.execute(scope) {
                println!("{e}");
                std::process::exit(1);
            };
        }
        Result::Err(e) => {
            // We hit a parsing error and print that out: (since the statement was wrong we cant even try to execute it)
            println!("{e}");
            std::process::exit(1);
        }
    }
}

/// gets called from Statements-'visitorpattern'
pub fn execute_block(parent_scope: Rc<Environment>, statements: Vec<Result<Statement, Err>>) {
    // create the new Scope:
    let local_scope = Rc::new(Environment::new(Some(parent_scope)));

    for statement in statements {
        exececute(local_scope.clone(), statement);
    }
}

/*
        Expressions Evaluate, something that evaluates to a value -> x+1 or true==nil
*/

// interface to evaluate our expressions. (1+3 resolves to 4) => we keep 4 "and throw the rest away"
trait Evaluates {
    fn evaluated(&self, env: Rc<Environment>) -> Result<Expr, Err>;
}

impl Expr {
    /// maps the visitor-patern like implementations of how diffferent expressions evaluate:
    pub fn evaluated(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        match self {
            Literal(expr) => expr.evaluated(env),
            Grouping(expr) => expr.evaluated(env),
            Unary(expr) => expr.evaluated(env),
            Binary(expr) => expr.evaluated(env),
            Logical(expr) => expr.evaluated(env),

            FnCall(expr) => expr.eval_with_env(env),

            VarAssign(expr) => expr.eval_with_env(env),
            VarRead(expr) => expr.eval_with_env(env),
        }
    }
}

impl VarAssignExpr {
    fn eval_with_env(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        let new_val = self.value.evaluated(env.clone());
        env.assign(self.name.clone(), new_val.clone()?)?;
        return new_val;
    }
}

impl VarReadExpr {
    fn eval_with_env(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        env.get_value(self.name.clone())
    }
}

impl FnCallExpr {
    fn eval_with_env(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        let callee = self.callee.evaluated(env.clone())?;
        let mut arguments = Vec::new();
        for arg in self.arguments.clone() {
            arguments.push(arg.evaluated(env.clone()))
        }
        // check if were trying to call function or obj not like "str".do()

        if let Expr::Literal(Value::Callable(function)) = callee.clone() {
            if arguments.len() != function.arity() {
                return Err(Err::Interpreter(
                    format!(
                        "Expected {} arguments but got {}.",
                        function.arity(),
                        arguments.len()
                    ),
                    69,
                ));
            }
            // Functions 'throw' on Return to get here, so we match for that special return-error
            match function.call(env.clone(), arguments) {
                Err(Err::ReturnValue(return_val)) => return Ok(return_val),
                res => return res,
                
            }
        }
        return Err(Err::Interpreter(
            format!("Can only call functions and classes."),
            69,
        ));
    }
}

impl Evaluates for Value {
    fn evaluated(&self, _env: Rc<Environment>) -> Result<Expr, Err> {
        return Ok(Literal(self.clone()));
    }
}

impl Evaluates for GroupingExpr {
    fn evaluated(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        return self.expr.evaluated(env);
    }
}

impl Evaluates for UnaryExpr {
    fn evaluated(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        let right = (*self.right).evaluated(env);

        match (self.token.clone(), right?) {
            (TokenType::Minus, Literal(Number(nr))) => Ok(Literal(Number(-nr))),
            (TokenType::Exclamation, Literal(Boolean(istrue))) => Ok(Literal(Boolean(!istrue))),
            // !nil = true :
            (TokenType::Exclamation, Literal(Nil)) => Ok(Literal(Boolean(true))),
            (token, _) => Err(Err::Interpreter(
                "NotImplementedUnaryExpr for :".to_string() + &token.to_string(),
                69,
            )),
        }
    }
}

impl Evaluates for BinaryExpr {
    fn evaluated(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        let left = (*self.left).evaluated(env.clone());
        let right = (*self.right).evaluated(env);

        match (left?, self.token.clone(), right?) {
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
            (left, token, right) => Err(Err::Interpreter(
                format!("NotImplementedBinaryExpr for {left} {token} {right}."),
                69,
            )),
        }
    }
}

impl Evaluates for LogicalExpr {
    fn evaluated(&self, env: Rc<Environment>) -> Result<Expr, Err> {
        let left = self.left.evaluated(env.clone())?;

        if self.token == TokenType::Or {
            if is_truthy(left.clone()) {
                return Ok(Expr::Literal(Value::Boolean(true)));
            }
        } else {
            // implicit TokenType::And
            if !is_truthy(left.clone()) {
                return Ok(Expr::Literal(Value::Boolean(false)));
            }
        }
        // we just want to return a Bool so we have to check truthiness of right side:
        let right = self.right.evaluated(env)?; // not evaluated if not reached! (ex no side-effects)

        match (self.token.clone(), is_truthy(right.clone())) {
            // a or b = false, true -> true
            (TokenType::Or, true) => Ok(Expr::Literal(Value::Boolean(true))),
            // a and b = true , true -> true
            (TokenType::And, true) => Ok(Expr::Literal(Value::Boolean(true))),
            _ => Ok(Expr::Literal(Value::Boolean(false))),
        }
    }
}

/*
            Helper Functions that handle some encapsulated logic
*/

// helper function to evaluate BinaryExpr:
fn subtraction(left: Expr, token: TokenType, right: Expr) -> Result<Expr, Err> {
    match (left, token, right) {
        (Literal(Number(l)), TokenType::Minus, Literal(Number(r))) => Ok(Literal(Number(l - r))),
        (left, token, right) => Err(Err::Interpreter(
            format!("FailedSubtraction for {left} {token} {right}."),
            69,
        )),
    }
}

// helper function to evaluate BinaryExpr:
fn multiplication(left: Expr, token: TokenType, right: Expr) -> Result<Expr, Err> {
    match (left, token, right) {
        (Literal(Number(l)), TokenType::Star, Literal(Number(r))) => Ok(Literal(Number(l * r))),
        (left, token, right) => Err(Err::Interpreter(
            format!("FailedMultiplication for {left} {token} {right}."),
            69,
        )),
    }
}

// helper function to evaluate BinaryExpr:
fn division(left: Expr, token: TokenType, right: Expr) -> Result<Expr, Err> {
    // explicit checking for division by 0 errors:
    if let Literal(Number(nr)) = right {
        if nr == 0.0 || nr == -0.0 {
            return Err(Err::Interpreter(
                format!("FailedMultiplication for {left} / 0"),
                69,
            ));
        }
    }

    match (left, token, right) {
        (Literal(Number(l)), TokenType::Slash, Literal(Number(r))) => Ok(Literal(Number(l / r))),
        (left, token, right) => Err(Err::Interpreter(
            format!("FailedDivision for {left} {token} {right}"),
            69,
        )),
    }
}

// helper function to evaluate BinaryExpr:
fn addition(left: Expr, token: TokenType, right: Expr) -> Result<Expr, Err> {
    match (left, token, right) {
        // addition
        (Literal(Number(l)), TokenType::Plus, Literal(Number(r))) => Ok(Literal(Number(l + r))),
        // string concatinations:
        (Literal(String(l)), TokenType::Plus, Literal(Number(r))) => {
            Ok(Literal(String(l + &r.to_string())))
        }
        (Literal(String(l)), TokenType::Plus, Literal(Boolean(r))) => {
            Ok(Literal(String(l + &r.to_string())))
        }
        (Literal(String(l)), TokenType::Plus, Literal(Nil)) => Ok(Literal(String(l + "Nil"))),
        (Literal(String(l)), TokenType::Plus, Literal(String(r))) => Ok(Literal(String(l + &r))),
        (left, token, right) => Err(Err::Interpreter(
            format!("FailedAddition for {left} {token} {right}"),
            69,
        )),
    }
}

// helper function to evaluate BinaryExpr:
fn comparison(left: Expr, token: TokenType, right: Expr) -> Result<Expr, Err> {
    match (left, token, right) {
        (Literal(Number(l)), TokenType::Less, Literal(Number(r))) => Ok(Literal(Boolean(l < r))),
        (Literal(Number(l)), TokenType::LessEqual, Literal(Number(r))) => {
            Ok(Literal(Boolean(l <= r)))
        }
        (Literal(Number(l)), TokenType::Greater, Literal(Number(r))) => Ok(Literal(Boolean(l > r))),
        (Literal(Number(l)), TokenType::GreaterEqual, Literal(Number(r))) => {
            Ok(Literal(Boolean(l >= r)))
        }
        (left, token, right) => Err(Err::Interpreter(
            format!("FailedComparison for {left} {token} {right}"),
            69,
        )),
    }
}

// helper function to evaluate BinaryExpr:
fn is_equal(left: Expr, token: TokenType, right: Expr) -> Result<Expr, Err> {
    match (left, token, right) {
        (l, TokenType::ExclamationEqual, r) => Ok(Literal(Boolean(l != r))),
        (l, TokenType::EqualEqual, r) => Ok(Literal(Boolean(l == r))),
        (left, token, right) => Err(Err::Interpreter(
            format!("FailedEqualityCheck for {left} {token} {right}"),
            69,
        )),
    }
}

// helper function to compare expression for truthiness: (ex: if "string" {...})
pub fn is_truthy(expr: Expr) -> bool {
    match expr {
        Expr::Literal(Value::Boolean(b)) => b,
        _ => return false,
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use crate::{lexer, parser::AST};


    use super::*;
    // some quick integration testing:
    fn test(input: &str, expected: Expr){
        //
        let global_scope = Rc::new(crate::environment::Environment::new(None));
        //
        let s = lexer::new_scanner(input);
        let (tokens, lexer_errs) = s.results();
        let ast = AST::new(tokens);
        let statements = ast.root;
        for s in statements{
            if let Ok(Statement::ExprSt(expr)) = s{
                let res = expr.evaluated(global_scope.clone());
                assert_eq!(res, Ok(expected.clone()));
            } 
            else { panic!("expected a Expression that evaluates!")}
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
        test("1 < 2;", Literal(Boolean(true)));
        test("10<=10;", Literal(Boolean(true)));
        test("10>10;", Literal(Boolean(false)));
        test("10>=10;", Literal(Boolean(true)));
        // test("true > false;", RuntimeErr(FailedComparison));
        // test("\"hello\" <= \"hello\";", RuntimeErr(FailedComparison));
    }

    #[test]
    fn addition_subtraction() {
        test("1.1+2;", Literal(Number(3.1)));
        test("1.2-2;", Literal(Number(-0.8)));
        test("\"hello\" + \" bye\";", Literal(String("hello bye".to_string())));
        test("\"hello_\" + nil;", Literal(String("hello_Nil".to_string())));
        test("\"hello_\" + 3;", Literal(String("hello_3".to_string())));
        test("\"hello_\" + true;", Literal(String("hello_true".to_string())));
        test("\"hello_\" + false;", Literal(String("hello_false".to_string())));
        // test("true+false;", RuntimeErr(FailedAddition));
    }

    #[test]
    fn division() {
        test("1/2;", Literal(Number(0.5)));
        test("-2/0.5;", Literal(Number(-4.0)));
        // test("1/0;", RuntimeErr(FailedDivisionByZero));
        // test("10 / nil;", RuntimeErr(FailedDivision));
        // test("true / 2;", RuntimeErr(FailedDivision));
    }

    #[test]
    fn multiplication() {
        test("1.1*2;", Literal(Number(2.2)));
        test("-1.2*0.2;", Literal(Number(-0.24)));
        // test("-1.2*nil;", RuntimeErr(FailedMultiplication));
    }
}
