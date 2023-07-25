/*
    The environment maps variable identifiers in our code to corresponding values.
*/

use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{types::{Err}, expressions::Expr};


/// Every local scope ex: {} gets its own map for local variables/functions
/// - to easily share the Environments we used a Rc (reference counted pointer) we can just clone
/// - to mutate in a Rc we need to use a RefCell inside of it. To make that part mutable
pub struct Environment{
    /// The parent Scope {} we 'live in' 
    pub enclosing: Option<Rc< Environment>>,     
    /// table to all our local variables
    pub values: RefCell<HashMap<String, Expr>>,              
}

impl Environment {
    pub fn new(enclosing: Option<Rc<Environment>>) -> Self {
        Environment {
            enclosing: enclosing, 
            values: RefCell::new(HashMap::new()),
        }
    }

    // type checking could happen here at runtime, but we are using dynamic types for now
    // if redefining variables was dissalowed it also could happen here ->

    /// define a variable like: 'var x = 12;' or 'var x;' -> x=nil
    /// - reassignment is allowed
    pub fn define(&self, name: String, val: Expr){
        self.values.borrow_mut().insert(name, val);
    }

    // read value of a variable like 'print x'
    pub fn get_value(&self, name: String) -> Result<Expr, Err> {
        match self.values.borrow_mut().get(&name){
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                // if we cant find it localy we try move up to parent scope:
                Some(encl_env) =>  encl_env.get_value(name),   
                None => Err(Err::Interpreter("Undefined Variable: [TODO access line nr] .".into(), 69)),
                //None => Err(Err::Interpreter("Undefined Variable: [".to_string()+name.lexeme+"] .", name.line)),
            }
        }
    }

    /// assings/reassigns value to previously declared variable: 'x = 123;'
    /// - not allowed to create a new variable (without 'var' keyword -> then define() )
    pub fn assign(&self, name: String, val: Expr) -> Result<(), Err> {
        if self.values.borrow_mut().contains_key(&name) {
            self.values.borrow_mut().insert(name, val);
            Ok(())
        } else {
            if let Some(enclosing_env) = &self.enclosing {
                enclosing_env.assign(name, val)
            } else {
                Err(Err::Interpreter("Can't write to Undefined Variable. [TODO access line nr]".to_string(), 69))
            }
        }
    }
}