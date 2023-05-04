/*
    The Tree-Walk Interpreter jlox
*/

use std::io::{self, Write};

use crate::lexer::new_scanner;
mod lexer;
mod parser;
mod token;

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
    let mut scanner = new_scanner(&input);
    scanner.scan_all_tokens();
    dbg!(scanner);
    //scanner.tokens = scanner.scan_tokens();

    //TODO: errors handling here
}
