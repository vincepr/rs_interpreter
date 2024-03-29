use std::rc::Rc;

use crate::{
    environment::Environment,
    interpreter::execute_block,
    statements::FunctionStatement,
    types::{Err, TokenType},
};

// Collection of all Expressions. They are the building blocks of our AST
// We expose those to our backend-interpreter AND middleend-parser

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    VarRead(VarReadExpr),
    VarAssign(VarAssignExpr),
    Logical(LogicalExpr),
    FnCall(FnCallExpr),
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

/// accesses a variable like 'x+1;' will have to access x
#[derive(Debug, Clone, PartialEq)]
pub struct VarReadExpr {
    pub name: String,
}

/// 'or' expression -> this shortcircuits (function calls have no side-effects)
/// so we handle them separate
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub token: TokenType,
    pub right: Box<Expr>,
}

/// writes to a local or global variable. 'x = 123;'
#[derive(Debug, Clone, PartialEq)]
pub struct VarAssignExpr {
    pub name: String,
    pub value: Box<Expr>,
}
impl VarAssignExpr {
    pub fn new(name: String, value: Expr) -> Self {
        VarAssignExpr {
            name: name,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCallExpr {
    pub callee: Box<Expr>,
    pub paren: TokenType,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    String(String),
    Number(f64),
    Callable(Rc<Function>),
    //Instance(Rc<Instance>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Native {
        arity: usize,
        func: fn() -> Result<Value, Err>,
    },
    Declared {
        function_st: FunctionStatement,
        closure: Rc<Environment>,
    },
}
impl Function {
    pub fn arity(&self) -> usize {
        match self {
            Function::Native { arity, func: _ } => *arity,
            Function::Declared {
                function_st,
                closure: _,
            } => {
                let FunctionStatement {
                    name: _,
                    params,
                    body: _,
                } = function_st;
                return params.len();
            }
        }
    }
    pub fn call(
        &self,
        _env: Rc<Environment>, // TODO: we probably can remove env? maybe do methods first? do they use this call?
        arguments: Vec<Result<Expr, Err>>,
    ) -> Result<Expr, Err> {
        match self {
            Function::Native { arity: _, func } => {
                // call() on Native functions just execuates the callback we stored in our map
                return Ok(Expr::Literal(func()?));
            }
            Function::Declared {
                function_st,
                closure,
            } => {
                // call() on Normal Functions (not Methods etc.)
                let FunctionStatement {
                    name: _name,
                    params,
                    body,
                } = function_st;
                let this_env = Rc::new(Environment::new(Some(Rc::clone(closure)))); // create new local-env for this function
                for i in 0..params.len() {
                    // we take arguments and write them to local env, so body can access them:
                    this_env.define(params[i].clone(), arguments[i].clone()?)
                }
                // we catch the upcoming return value wrapped in an error
                if let Err(Err::ReturnValue(val)) = execute_block(this_env, body.clone()) {
                    return Ok(val);
                }
                return Ok(Expr::Literal(Value::Nil)); // or use default nil if no return value
            }
        }
    }
}

// Display Trait used for pretty-printing the ast tree:
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //Expr::ErrorExpr => f.write_str("ErrorExpr"),
            Expr::Literal(Value::Boolean(b)) => b.fmt(f),
            Expr::Literal(Value::Nil) => f.write_str("nil"),
            Expr::Literal(Value::String(s)) => s.fmt(f),
            Expr::Literal(Value::Number(n)) => n.fmt(f),
            Expr::Literal(Value::Callable(n)) => write!(f, "{:#?}", n),

            Expr::Binary(BinaryExpr { left, token, right }) => {
                f.write_fmt(format_args!("<{left} {token} {right}>"))
            }
            Expr::Unary(UnaryExpr { token, right }) => {
                f.write_fmt(format_args!("<{token} {right}>"))
            }
            Expr::Logical(LogicalExpr { left, token, right }) => {
                f.write_fmt(format_args!("<{left} {token} {right}>"))
            }
            Expr::Grouping(GroupingExpr { expr }) => f.write_fmt(format_args!("({expr})")),
            Expr::VarRead(VarReadExpr { name }) => name.fmt(f),
            Expr::VarAssign(VarAssignExpr { name, value }) => {
                f.write_fmt(format_args!("<{name} = {value}>"))
            } //Expr::RuntimeErr(e) => write!(f, "RuntimeErr({:?})", e),
            Expr::FnCall(FnCallExpr {
                callee,
                paren: _,
                arguments: _,
            }) => f.write_fmt(format_args!("{callee}()")), //_ => write!(f, "{:?}", self),             //Failback to Debug-Printing for unimplemented expressions?
        }
    }
}
