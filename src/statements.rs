/*
    Statements are similar to expressions. 
    They build the other Tree-Like strure of our interpreter.
    But instead of evaluating to some value they deal things like:
        - binding identifiers(/names) to data and functions
        - state and or side effects
        - Represent blocks and local scope
*/

use crate::{expressions::Expr, interpreter::Evaluates};


#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExprSt(Expr),
    PrintSt(Expr),
}
impl Statement {
    pub fn eval(&self) {
        match self {
            Statement::ExprSt(expr) => eval_expr_statement(expr),
            Statement::PrintSt(expr) => eval_print_statement(expr),
        }
    }
}
fn eval_print_statement(expr: &Expr){
    let res = expr.evaluated();
    println!("{res}");  // create the side-effect of print"..."
}
fn eval_expr_statement(expr: &Expr){
    expr.evaluated();
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