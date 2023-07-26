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

use crate::{types::Err, environment::Environment, expressions::Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExprSt(Expr),
    PrintSt(Expr),
    VariableSt(String, Expr),
    BlockSt(Vec<Result<Statement, Err>>),
    //ErrStatementVariable,
}

impl Statement {
    /// visitor-like pattern that maps each Statment to its handler:
    pub fn execute(self, current_env: Rc<Environment>) -> Result<(), Err> {
        match self {
            Statement::ExprSt(expr) => execute_expr_statement(expr, current_env),
            Statement::PrintSt(expr) => execute_print_statement(expr, current_env),
            Statement::VariableSt(name, initial_value) => {
                execuate_var_statement(name, initial_value, current_env)
            }
            //Statement::ErrStatementVariable => panic!("Hit Error Statement Variable"),
            Statement::BlockSt(statements) => {
                execute_block_statement(statements, current_env)
            }
        }
    }
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

fn execuate_var_statement(name: String, initial_value: Expr, environment: Rc<Environment>) -> Result<(), Err> {
    // uninitialized will pass down a nil -> so they become nil;
    let value = initial_value.evaluated(environment.clone())?;
    environment.define(name, value);
    Ok(())
}

fn execute_block_statement(statements: Vec<Result<Statement, Err>>, env: Rc<Environment>) -> Result<(), Err> {
    crate::interpreter::execute_block(env, statements);
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
