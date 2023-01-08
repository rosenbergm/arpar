use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

// use termion::color;
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;

use lexer::Lexer;
use parser::{Parse, Parser};
use postfix::PostfixParser;

mod lexer;
mod parser;
mod postfix;

enum OperationMode {
    Infix,
    Postfix,
}

fn evaluate(
    mode: &OperationMode,
    line: String,
    memory: &HashMap<String, String>,
) -> Result<(), String> {
    let lexer = Lexer::new(line);

    let expression = match mode {
        OperationMode::Infix => Parser::new(&lexer).run()?,
        OperationMode::Postfix => PostfixParser::new(&lexer).run()?,
    };

    match expression.evaluate(memory) {
        Ok(result) => {
            println!("{}", result);

            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn get_line(mode: &OperationMode) -> String {
    match mode {
        OperationMode::Infix => {
            print!("INFIX > ");
        }
        OperationMode::Postfix => {
            print!("POSTFIX > ");
        }
    }

    io::stdout().flush().unwrap();

    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_s) => {}
        Err(_e) => {}
    };

    input.trim().to_string()
}

fn run_repl(mode: OperationMode) -> Result<(), String> {
    let mut memory: HashMap<String, String> = HashMap::new();

    println!(
        "
Hello and welcome! If you type in an expression according to this grammar

    expr -> factor + expr | factor
    factor -> term * factor | term
    term -> NUMBER | ( expr ) | VARIABLE

the program will be happy.

You can quit the REPL by typing `exit`.
You can list all variables by typing `defined`.
            "
    );

    loop {
        let line = get_line(&mode);

        if line == "exit" {
            return Ok(());
        }

        if line == "defined" {
            println!("Here is a list of defined variables:");

            if memory.is_empty() {
                println!("There are no defined variables :(");
            }

            for var in &memory {
                println!("{} = {}", var.0, var.1);
            }

            continue;
        }

        if line.contains('=') {
            let split_line: Vec<String> = line
                .clone()
                .split('=')
                .map(|el| el.trim().to_string())
                .collect();

            if split_line.len() != 2 {
                println!("Error: Incorrect variable assignment.");
                continue;
            }

            let variable_name = split_line.get(0).unwrap().clone();

            if variable_name.len() != 1 {
                println!("Error: Variable name has to be one character long, [a-z].");
                continue;
            }

            let variable_expr = split_line.get(1).unwrap().clone();

            println!("Assignment: {} = {}", variable_name, variable_expr);

            memory.insert(variable_name, variable_expr);
            continue;
        }

        if let Err(e) = evaluate(&mode, line, &memory) {
            println!("Error: {}", e);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode_arg = args.get(1);

    let mode: OperationMode = match mode_arg {
        Some(mode_flag) => match mode_flag.as_str() {
            "--postfix" => OperationMode::Postfix,
            "--infix" => OperationMode::Infix,
            _ => {
                eprintln!("Initialization error.");
                std::process::exit(1);
            }
        },
        _ => {
            println!("No mode specified, defaulting to infix. To change, pass either --infix or --postfix");
            OperationMode::Infix
        }
    };

    if let Err(error) = run_repl(mode) {
        eprintln!("Unexpected error: {}", error)
    }
}
