/*
    Statements are similar to expressions.
    They build the other Tree-Like strure of our interpreter.
    But instead of evaluating to some value they deal things like:
        - each statement ends with a ';' mostly.
        - binding identifiers(/names) to data and functions
        - state and or side effects
        - Represent blocks and local scope
*/

use std::rc::Rc;

use crate::{
    environment::Environment,
    expressions::{Expr, Function, Value},
    interpreter::{execute_block, is_truthy},
    types::Err,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExprSt(Expr),
    PrintSt(Expr),
    VariableSt(String, Expr),
    BlockSt(Vec<Result<Statement, Err>>),
    IfSt {
        condition: Expr,
        then_: Box<Statement>,
        else_: Option<Box<Statement>>,
    },
    While {
        condition: Expr,
        body: Box<Statement>,
    },
    FunctionSt(FunctionStatement),
    ReturnSt {
        keyword: String,
        value: Expr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Result<Statement, Err>>,
}

impl Statement {
    /// visitor-like pattern that maps each Statment to its handler:
    pub fn execute(self, current_env: Rc<Environment>) -> Result<(), Err> {
        match self {
            Self::ExprSt(expr) => execute_expr_statement(expr, current_env),
            Self::PrintSt(expr) => execute_print_statement(expr, current_env),
            Self::VariableSt(name, initial_value) => {
                execuate_var_statement(name, initial_value, current_env)
            }
            Self::BlockSt(statements) => execute_block_statement(statements, current_env),
            Self::IfSt {
                condition,
                then_,
                else_,
            } => execute_if_statement(condition, then_, else_, current_env),
            Self::While { condition, body } => {
                execute_while_statement(condition, *body, current_env)
            }
            Self::FunctionSt(fn_st) => execute_function_statement(fn_st, current_env),
            Self::ReturnSt { keyword, value } => {
                execute_return_statement(keyword, value, current_env)
            }
        }
    }
}

fn execute_return_statement(
    _keyword: String,
    value: Expr,
    env: Rc<Environment>,
) -> Result<(), Err> {
    let mut return_val = Expr::Literal(Value::Nil);
    if value != Expr::Literal(Value::Nil) {
        return_val = value.evaluated(env)?;
    }
    Err(Err::ReturnValue(return_val))
}

/// a function is declared 'fun name(...params){ ...body; }'
fn execute_function_statement(fn_st: FunctionStatement, env: Rc<Environment>) -> Result<(), Err> {
    let FunctionStatement { name, .. } = fn_st.clone();
    let function = Expr::Literal(Value::Callable(Rc::new(Function::Declared {
        function_st: fn_st,
        closure: Rc::clone(&env),
    })));
    env.define(name, function);
    Ok(())
}

fn execute_while_statement(
    condition: Expr,
    body: Statement,
    env: Rc<Environment>,
) -> Result<(), Err> {
    while is_truthy(condition.evaluated(env.clone())?) {
        body.clone().execute(env.clone())?;
    }
    Ok(())
}

fn execute_expr_statement(expr: Expr, env: Rc<Environment>) -> Result<(), Err> {
    expr.evaluated(env)?; // our Trait-interface that will evaluate it down recursively
    Ok(())
}

fn execute_print_statement(expr: Expr, env: Rc<Environment>) -> Result<(), Err> {
    let res = expr.evaluated(env)?;
    println!("{res}"); // create the side-effect of print"res..."
    Ok(())
}

fn execuate_var_statement(
    name: String,
    initial_value: Expr,
    environment: Rc<Environment>,
) -> Result<(), Err> {
    // uninitialized will pass down a nil -> so they become nil;
    let value = initial_value.evaluated(environment.clone())?;
    environment.define(name, value);
    Ok(())
}

fn execute_block_statement(
    statements: Vec<Result<Statement, Err>>,
    env: Rc<Environment>,
) -> Result<(), Err> {
    let _ = execute_block(env, statements); // it is only possible to return from functions in lox
    Ok(())
}

fn execute_if_statement(
    condition: Expr,
    then_: Box<Statement>,
    else_: Option<Box<Statement>>,
    env: Rc<Environment>,
) -> Result<(), Err> {
    if is_truthy(condition.evaluated(env.clone())?) {
        then_.execute(env)?;
    } else if let Some(else_branch) = else_ {
        else_branch.execute(env)?;
    }
    Ok(())
}

// fn eval_assign_statement(name: String, new_value: Expr ,  env: &mut Environment) {
//     // uninitialized will pass down a nil -> so they become nil;
//     let value = new_value.evaluated(env);
//     env.assign(name, value);
// }

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{:?}", self), //Failback to Debug-Printing for unimplemented ones:
        }
    }
}
