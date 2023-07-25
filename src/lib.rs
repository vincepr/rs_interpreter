/*
    The Tree-Walk Interpreter jlox
*/

use std::io::{self, Write};

use crate::{lexer::new_scanner, parser::AST};
mod environment;
mod expressions;
mod interpreter;
mod lexer;
mod parser;
mod statements;
mod types;

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
        run(input, true);
    }
}

pub fn run_file(input: String, print_ast: bool) {
    run(input, print_ast);
}

fn run(input: String, print_ast: bool) {
    let lexer = new_scanner(&input);
    let (tokens, mut errors) = lexer.results();

    let ast = AST::new(tokens);

    //optional debug info (prints the ast prefore interpreting it)
    if print_ast {
        println!("AST: {}", ast.print());
    }
    let expr = ast.root;
    interpreter::interpret(expr);

    // join errors together and print them out:
    errors.extend(ast.errors);
    for er in errors {
        println!("{}", er.to_string());
    }
}
