use std::collections::HashMap;
use std::env;

use repl::CommandHinter;
use rustyline::{Editor, EventHandler, KeyEvent};

use lexer::Lexer;
use parser::{Parse, Parser};
use postfix::PostfixParser;

use crate::repl::{command_hints, TabEventHandler};

mod lexer;
mod parser;
mod postfix;
mod repl;

enum OperationMode {
    Infix,
    Postfix,
}

fn evaluate(
    mode: &OperationMode,
    line: String,
    memory: &HashMap<String, String>,
) -> Result<(), String> {
    let tokens = Lexer::run(line)?;

    let expression = match mode {
        OperationMode::Infix => Parser::run(&tokens)?,
        OperationMode::Postfix => PostfixParser::run(&tokens)?,
    };

    match expression.evaluate(memory) {
        Ok(result) => {
            println!("{}", result);

            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn get_line(editor: &mut Editor<CommandHinter>, mode: &OperationMode) -> rustyline::Result<String> {
    let prompt = match mode {
        OperationMode::Infix => "INFIX > ",
        OperationMode::Postfix => "POSTFIX > ",
    };

    let input = editor.readline(prompt)?;

    Ok(input.trim().to_string())
}

fn run_repl(mode: OperationMode) -> rustyline::Result<()> {
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

    let mut memory: HashMap<String, String> = HashMap::new();

    let helper = CommandHinter {
        hints: command_hints(),
    };
    let mut editor: Editor<CommandHinter> = Editor::new()?;

    editor.set_helper(Some(helper));

    editor.bind_sequence(
        KeyEvent::from('\t'),
        EventHandler::Conditional(Box::new(TabEventHandler)),
    );

    loop {
        let line = get_line(&mut editor, &mode)?;

        editor.add_history_entry(line.as_str());

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
