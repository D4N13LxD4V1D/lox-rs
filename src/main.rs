#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::fs;
use std::io::Write;

mod error_handling;
mod scanner;
mod token;
mod ast;
mod interpreter;
mod environment;

fn run(source: &str, env: &mut environment::Environment) {
    let tokens = scanner::scan(source);
    let stmts = ast::parse(tokens);

    //ast::print(&stmts);
    //println!();
    interpreter::execute(&stmts, env);
}

fn run_file(args: Vec<String>) {
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    for line in contents.split_terminator('\n') {
        run(&line, &mut environment::Environment::new());
    }
}

fn run_prompt() {
    let mut env = environment::Environment::new();
    loop {
        let mut input = String::new();
        print!("\x1b[1;37m>>> \x1b[0m");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        run(&input, &mut env);
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
