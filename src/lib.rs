/*
    The Tree-Walk Interpreter jlox
*/

use std::io::{self, Write};

use crate::{interpreter::Evaluates, lexer::new_scanner, parser::AST};
mod expressions;
mod interpreter;
mod lexer;
mod parser;
mod types;

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
    println!("AST: {}", ast.print());

    let expr = ast.root;
    let result = interpreter::interpret(expr);
    println!("{result}");

    // join errors together:
    errors.extend(ast.errors);
    // print out the Errors:
    for er in errors {
        println!("{}", er.to_string());
    }
}
