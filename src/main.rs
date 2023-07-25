use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // running as "sanbox-mode"
        rs_interpreter::run_prompt();   // this always runs in debug mode
    } else if args.len() == 2 {
        // running a file from path
        if let Some(source_code) = open_file(&args[1]) {
            rs_interpreter::run_file(source_code, false);
        } else {
            process::exit(1);
        }
    } else if args.len() == 3 && args[2] == "--debug" {
        // debug flag to print out ast
        if let Some(source_code) = open_file(&args[1]) {
            rs_interpreter::run_file(source_code, true);
        } else {
            process::exit(1);
        }
    } else {
        println!("Usage: rlox [optional: PathToFile] [optional: --debug] ");
        process::exit(1);
    }
}

fn open_file(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}
