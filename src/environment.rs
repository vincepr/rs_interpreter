/*
    The environment maps variable identifiers in our code to corresponding values.
*/

use std::collections::HashMap;

use crate::{types::{Err,Token}, expressions::Expr};


/// Every local scope ex: {} gets its own map for local variables/functions
pub struct Environment<'a>{
    /// The parent Scope {} we 'live in' 
    enclosing: Option<&'a Environment<'a>>,     
    /// table to all our local variables
    values: HashMap<String, Expr>,              
}

impl <'a> Environment <'a>{
    pub fn new(enclosing: Option<&'a Environment>) -> Self{
        Environment {
            enclosing: enclosing,
            values: HashMap::new(),
        }
    }

    // type checking could happen here at runtime, but we are using dynamic types for now
    // if redefining variables was dissalowed it also could happen here ->

    /// define a variable like: 'var x = 12;' or 'var x;' -> x=nil
    /// - reassignment is allowed
    pub fn define(&mut self, name: String, val: Expr){
        self.values.insert(name, val);
    }

    // read value of a variable like 'print x'
    pub fn get_value(&self, name: &Token) -> Result<Expr, Err> {
        match self.values.get(name.lexeme){
            Some(val) => Ok(val.clone()),
            None => match self.enclosing {
                // if we cant find it localy we try move up to parent scope:
                Some(enclEnv) =>  enclEnv.get_value(name),   
                None => Err(Err::Interpreter("Undefined Variable: [".to_string()+name.lexeme+"] .", name.line)),
            }
        }
    }

    /// assings/reassigns value to previously declared variable: 'x = 123;'
    /// - not allowed to create a new variable (without 'var' keyword -> then define() )
    pub fn assign(&self, name: &Token, val: Expr) -> Result<(), Err> {
        if self.values.contains_key(name.lexeme) {
            self.values.insert(name.lexeme.to_string(), val);
            Ok(())
        } else {
            match self.enclosing {
                Some(enclEnv) => enclEnv.assign(name, val),
                None => Err(Err::Interpreter("Can't write to Undefined Variable: [".to_string()+name.lexeme+"] .", name.line)),
            }
        }
    }
}