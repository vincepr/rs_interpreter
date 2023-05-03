use std::{env, fs, process};

fn main() {
    println!("Running the LOX interpreter:");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // running as "sanbox-mode"
        rs_interpreter::run_prompt();
    } else if args.len() == 2 {
        // running a file from path
        if let Some(source_code) = open_file(&args[1]) {
            rs_interpreter::run_file(source_code);
        } else {
            process::exit(1);
        }
    } else {
        process::exit(1);
    }
}

fn open_file(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}
