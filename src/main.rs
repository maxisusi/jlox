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
        println!("{}, {:?}, {}", token.lexeme, token.token_type, token.line);
    }
}

fn run_file(path: &String) {
    let file = fs::read_to_string(path).expect("Could not read the file");
    run(file)
}

fn run_prompt() {
    loop {
        let mut prompt_input = String::new();
        io::stdin().read_line(&mut prompt_input).unwrap();

        if prompt_input.contains("exit()") {
            exit(0);
        }
        println!("You typed: {}", prompt_input);
    }
}
