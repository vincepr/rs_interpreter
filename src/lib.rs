/*
    The Tree-Walk Interpreter jlox
*/

use std::io::{self, Write};

use crate::{lexer::new_scanner, parser::AST};
mod expressions;
mod interpreter;
mod lexer;
mod parser;
mod types;
mod statements;
mod environment;

pub fn run_prompt() {
    println!("Interpreter running, input a line:");
    loop {
        print!(">");
        io::stdout().flush().expect("flush failed!");
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

    let ast = AST::new(tokens);

    //TODO: make printing of AST optional (flag or command in REPL to toggle)
    println!("AST: {}", ast.print());
    let expr = ast.root;
    interpreter::interpret(expr);

    // join errors together and print them out:
    errors.extend(ast.errors);
    for er in errors {
        println!("{}", er.to_string());
    }
}
