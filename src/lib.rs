/*
    The Tree-Walk Interpreter jlox
*/

use std::io::{self, Write};

use crate::{lexer::new_scanner, parser::AST};
mod lexer;
mod parser;
mod token;

#[derive(Debug, Clone)]
pub enum Err {
    // WhatFailed (Error-message, line-of-error)
    //      TODO: could change to point to character/byte of error
    //      -> re-evaluate to line && character in line
    Parser(String, usize),
    Lexer(String, usize),
}
impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::Lexer(message, line) => {
                f.write_fmt(format_args!("Lexer-ERROR in line: {line} : {message}!"))
            }
            Err::Parser(message, line) => {
                f.write_fmt(format_args!("ParserERROR in line: {line} : {message}!"))
            }
        }
    }
}

pub fn run_prompt() {
    println!("Interpreter running, input a line:");
    loop {
        // read user input
        print!(">");
        io::stdout().flush().expect("flush failed!"); // rust stdin is buffered so we have to flush it
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.starts_with("#exit") {
            break;
        }
        run(input);
    }
}

pub fn run_file(input: String) {
    run(input);
}

fn run(input: String) {
    let lexer = new_scanner(&input);
    let (tokens, mut errors) = lexer.results();
    // scanner.scan_all_tokens();
    // let tokens = scanner.return_tokens();

    let ast = AST::new(tokens);

    // print out the AST:
    println!("{}", ast.print());

    for er in errors {
        println!("{}", er.to_string());
    }
    for er in ast.errors {
        println!("{}", er.to_string());
    }


    // // join errors together:
    // errors.extend(ast.errors);
    // // print out the Errors:
    // for er in errors {
    //     println!("{}", er.to_string());
    // }
}
