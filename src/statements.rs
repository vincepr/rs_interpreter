/*
    Statements are similar to expressions. 
    They build the other Tree-Like strure of our interpreter.
    But instead of evaluating to some value they deal things like:
        - each statement ends with a ';' mostly.
        - binding identifiers(/names) to data and functions
        - state and or side effects
        - Represent blocks and local scope
*/

use crate::{expressions::Expr, interpreter::Evaluates, environment::{Environment}};


#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExprSt(Expr),
    PrintSt(Expr),
    VariableSt(String, Expr),
    AssignSt(String, Expr),
    BlockSt(Vec<Statement>),
    ErrStatementVariable,
}

pub struct AssignSt <'a> (crate::types::Token <'a>, Expr);

impl Statement {
    /// visitor-like pattern that maps each Statment to its handler:
    pub fn execute(self, currentEnv: &mut Environment) {
        match self {
            Statement::ExprSt(expr) => eval_expr_statement(expr),
            Statement::PrintSt(expr) => eval_print_statement(expr),
            Statement::VariableSt(name, initialValue) => eval_var_statement(name, initialValue, currentEnv),
            Statement::AssignSt(name, newValue) => eval_assign_statement(name, newValue, currentEnv),
            Statement::ErrStatementVariable => panic!("Hit Error Statement Variable"),
            Statement::BlockSt(statements) => {
                panic!("//TODO: statments.rs BlockSt")
            },
        }
    }
}
fn eval_expr_statement(expr: Expr){
    expr.evaluated();   // our Trait-interface that will evaluate it down recursively
}
fn eval_print_statement(expr: Expr){
    let res = expr.evaluated();
    println!("{res}");  // create the side-effect of print"res..."
}
fn eval_var_statement(name: String, initialValue: Expr ,  environment: &mut Environment) {
    // uninitialized will pass down a nil -> so they become nil;
    let value = initialValue.evaluated();
    environment.define(name, value);
}
fn eval_block_statement(statements: Vec<Statement>,  environment: &mut Environment) {
    crate::interpreter::executeBlock(environment, statements);
}
fn eval_assign_statement(name: String, newValue: Expr ,  environment: &mut Environment) {
    // uninitialized will pass down a nil -> so they become nil;
    let value = newValue.evaluated();
    environment.assign(name, value);
}




#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{:?}", self),             //Failback to Debug-Printing for unimplemented ones:
        }
    }
}