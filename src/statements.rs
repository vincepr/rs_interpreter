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

use crate::{environment::Environment, expressions::Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExprSt(Expr),
    PrintSt(Expr),
    VariableSt(String, Expr),
    BlockSt(Vec<Statement>),
    ErrStatementVariable,
}

impl Statement {
    /// visitor-like pattern that maps each Statment to its handler:
    pub fn execute(self, current_env: Rc<Environment>) {
        match self {
            Statement::ExprSt(expr) => eval_expr_statement(expr, current_env),
            Statement::PrintSt(expr) => eval_print_statement(expr, current_env),
            Statement::VariableSt(name, initial_value) => {
                eval_var_statement(name, initial_value, current_env)
            }
            Statement::ErrStatementVariable => panic!("Hit Error Statement Variable"),
            Statement::BlockSt(statements) => {
                eval_block_statement(statements, current_env);
            }
        }
    }
}
fn eval_expr_statement(expr: Expr, env: Rc<Environment>) {
    expr.evaluated(env); // our Trait-interface that will evaluate it down recursively
}
fn eval_print_statement(expr: Expr, env: Rc<Environment>) {
    let res = expr.evaluated(env);
    println!("{res}"); // create the side-effect of print"res..."
}
fn eval_var_statement(name: String, initial_value: Expr, environment: Rc<Environment>) {
    // uninitialized will pass down a nil -> so they become nil;
    let value = initial_value.evaluated(environment.clone());
    environment.define(name, value);
}
fn eval_block_statement(statements: Vec<Statement>, env: Rc<Environment>) {
    crate::interpreter::execute_block(env, statements);
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
