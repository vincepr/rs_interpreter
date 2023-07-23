/*
    The environment maps variable identifiers in our code to corresponding values.
*/

use std::collections::HashMap;

use crate::types::{Err,Token};

#[derive(Debug,Clone)]
enum VarType{
    Nil,
    String(String),
    Number(f64),
    Boolean(bool),
}

pub struct VarMap( HashMap<String,VarType> );

impl VarMap{
    pub fn new() -> Self{
        VarMap(HashMap::new())
    }

    // type checking could happen here at runtime, but we are using dynamic types for now
    // if redefining variables was dissalowed it also could happen here ->

    /// define a variable like: 'var x = 12;'
    /// - reassignment is allowed
    pub fn define(&mut self, name: String, val: VarType){
        self.0.insert(name, val);
    }

    pub fn get_value(&self, token: &Token) -> Result<VarType, Err> {
        let VarMap(map) = self;
        match map.get(token.lexeme){
            Some(val) => Ok(val.clone()),
            None =>Err(Err::Interpreter("Undefined Variable: [".to_string()+token.lexeme+"] .", token.line)),
        }
    }
}