mod structures;
mod constants;
mod structures_test;

use std::fs::read;
use std::io::Write;
use crate::structures::TransactionStatus;

enum StatementStatus {
        Success,
        Unknown,
        Exit,
        Failure
}

struct Statement {
    statement: String,
    arguments: Option<Vec<String>>,
    is_meta: bool
}



fn main() {

    let mut input = String::new();
    let mut table:structures::Table = structures::create_table();

    loop {
        print!("db> ");

        let input_stmnt = read_input(&mut input);
        match input_stmnt {
            Some(input_stmnt) => {
                if input_stmnt.is_meta {
                    match process_meta_stmnt(input_stmnt) {
                        StatementStatus::Exit => {
                            break;
                        },
                        _ => (),
                    }
                }else{
                    let _ = process_stmnt(&mut table, input_stmnt);
                }
            },
            None => {}
        }

        input.clear();
    }
}

fn read_input(input: &mut String) -> Option<Statement> {
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(input)
        .expect("Failed to read line");

    let processed_input: &str = &input.trim()[..];
    let tokens: Vec<&str> = processed_input.split(' ').collect();
    return process_input_tokens(tokens);
}

fn process_input_tokens(tokens: Vec<&str>) -> Option<Statement> {
    if tokens.len() <= 1 {
        if tokens[0] == "" {
            return None
        }
        return Some(Statement {
            statement: String::from(tokens[0]),
            arguments: None,
            is_meta: tokens[0].chars().nth(0).unwrap() == '.'
        });
    }
    return Some(Statement {
        statement: String::from(tokens[0]),
        arguments: Some(tokens[1..].iter().map(|&x| String::from(x)).collect()),
        is_meta: tokens[0].chars().nth(0).unwrap() == '.'
    })
}

fn process_meta_stmnt(stmnt: Statement) -> StatementStatus {
    // TODO: Handle out of bound index exception
    match stmnt.statement.as_str() {
            constants::EXIT_CMD => {
                println!("Exiting the program");
                return StatementStatus::Exit;
            },
            _ => { 
                println!("Unknown meta statement: {}", stmnt.statement);
                return StatementStatus::Unknown;
            }
    }
}

fn process_stmnt(table:&mut structures::Table, stmnt: Statement) -> StatementStatus {
    // TODO: Handle out of bound index exception
    match stmnt.statement.as_str() {
             constants::INSERT_STMNT => {
                println!("Insert stmnt");
                return process_insert_stmnt(table, stmnt);
            },
             constants::SELECT_STMNT => {
                println!("Select stmnt");
                return process_select_stmnt(table, stmnt);
            },
            _ => { 
                println!("Unknown statement: {}", stmnt.statement);
                return StatementStatus::Unknown
            }
    }
}


fn process_insert_stmnt(table:&mut structures::Table, stmnt: Statement) -> StatementStatus {
    let stmnt_arguments = stmnt.arguments;
    match stmnt_arguments {
        Some(stmnt_arguments) => {
            if stmnt_arguments.len() != 3 {
                return StatementStatus::Failure;
            }
            match table.insert_row(structures::Row {
                id: stmnt_arguments[0].parse().unwrap(),
                username: stmnt_arguments[1].clone(),
                email: stmnt_arguments[2].clone()
            }) {
                structures::TransactionStatus::Success => { return StatementStatus::Success; },
                _  => { return StatementStatus::Failure }
            };
        },
        None => { return StatementStatus::Failure; }
    }
}

fn process_select_stmnt(table:&mut structures::Table, stmnt: Statement) -> StatementStatus {
    let stmnt_arguments = stmnt.arguments;
    match stmnt_arguments {
        Some(stmnt_arguments) => {
            if stmnt_arguments.len() != 1 {
                return StatementStatus::Failure;
            }
            let r:Option<&structures::Row> = table.select_row(stmnt_arguments[0].parse().unwrap());
            println!("Row {:?} : {:?}", stmnt_arguments[0], r);
            return StatementStatus::Success;
        },
        None => { return StatementStatus::Failure; }
    }
}