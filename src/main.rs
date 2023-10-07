mod lox_error;
mod scanner;
mod token;
mod token_type;
use std::fs;
use std::io::{self};
use std::{env, process::exit};

fn main() {
    // Collect the arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        let path = &args[1];
        run_file(path);
    } else {
        run_prompt();
    }
}

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // Print the tokens
    for token in tokens {
        println!("{:?}", token);
    }
}

fn run_file(path: &String) {
    match fs::read_to_string(path) {
        Ok(e) => run(e),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_prompt() {
    loop {
        let mut prompt_input = String::new();
        io::stdin().read_line(&mut prompt_input).unwrap();
        run(prompt_input)
    }
}
