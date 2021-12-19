mod structures;
mod constants;
mod structures_test;

use std::io::Write;

const EXIT_CMD: &str = ".exit";
const INSERT_STMNT: &str = "insert";
const SELECT_STMNT: &str = "select";

enum MetaCmdStatus {
        Unknown,
        Exit,
}

enum StatementStatus {
        Success,
        Unknown,
        Failure
}

fn main() {

    let mut input = String::new();

    loop {
        print!("db> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let processed_input: &str = &input.trim()[..];
        let tokens: Vec<&str> = processed_input.split(' ').collect();

        if tokens.len() > 0 && processed_input != "" {
            let is_meta_cmd: bool = tokens[0].chars().nth(0).unwrap() == '.';
            if is_meta_cmd {
                match process_meta_cmd(tokens) {
                    MetaCmdStatus::Exit => {
                        break;
                    },
                    _ => (),
                }
            }else{
                let (stmnt_status, row) = process_statement(tokens);
                match stmnt_status {
                    StatementStatus::Success => {
                        println!("Row: {:?}", row);
                    },
                    _ => ()
                }
            }
        }

        input.clear();
    }
}

fn process_meta_cmd(cmd: Vec<&str>) -> MetaCmdStatus {
    // TODO: Handle out of bound index exception
    match cmd[0] {
            EXIT_CMD => { 
                println!("Exiting the program");
                return MetaCmdStatus::Exit;
            },
            _ => { 
                println!("Unknown command: {}", cmd[0]);
                return MetaCmdStatus::Unknown;
            }
    }
}

fn process_statement(stmnt: Vec<&str>) -> (StatementStatus, Option<structures::Row>) {
    // TODO: Handle out of bound index exception
    match stmnt[0] {
             INSERT_STMNT => { 
                println!("Insert stmnt");
                if stmnt.len() == 4 {
                    return (StatementStatus::Success, Some(
                        // TODO: Not introducing correct type panics
                            structures::Row {
                                id: stmnt[1].parse::<i32>().unwrap(), 
                                username: String::from(stmnt[2]), 
                                email: String::from(stmnt[3]) 
                            })
                        );
                }
                return (StatementStatus::Failure, None);
            },
             SELECT_STMNT => { 
                println!("Select stmnt");
                return (StatementStatus::Success, None);
            },
            _ => { 
                println!("Unknown statement: {}", stmnt[0]);
                return (StatementStatus::Unknown, None);
            }
    }
}

