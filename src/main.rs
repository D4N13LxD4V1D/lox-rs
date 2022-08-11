#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::fs;
use std::io::Write;

pub mod error_handling;
pub mod scanner;
pub mod token;
pub mod ast;
pub mod expression;
pub mod interpreter;

fn run(source: &str) {
    let tokens = scanner::scan(source);
    let expr = ast::parse(tokens);

    ast::print(&expr);
    println!();
    interpreter::print(&expr);
}

fn run_file(args: Vec<String>) {
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    for line in contents.split_terminator('\n') {
        run(&line);
    }
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("\x1b[1;37m>>> \x1b[0m");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        run(&input);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        run_file(args);
    } else {
        run_prompt();
    }
}
