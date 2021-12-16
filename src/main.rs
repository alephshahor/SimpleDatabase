use std::io::Write;

const EXIT_CMD: &str = ".exit";
const INSERT_STMNT: &str = "insert";
const SELECT_STMNT: &str = "select";

enum MetaCmdStatus {
        Success,
        Unknown,
        Exit,
}

enum StatementStatus {
        Success,
        Unknown,
        Exit,
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

        let is_meta_cmd: bool = input.chars().nth(0).unwrap() == '.';
        if(is_meta_cmd) {
            match process_meta_cmd(processed_input) {
                MetaCmdStatus::Exit => {
                    break;
                },
                _ => (),
            }
        }else{
            match process_statement(processed_input) {
                _ => (),
            }
        }

        input.clear();
    }
}

fn process_meta_cmd(cmd: &str) -> MetaCmdStatus {
    match cmd {
            EXIT_CMD => { 
                println!("Exiting the program");
                return MetaCmdStatus::Exit;
            },
            _ => { 
                println!("Unknown command: {}", cmd);
                return MetaCmdStatus::Unknown;
            }
    }
    return MetaCmdStatus::Success;
}

fn process_statement(stmnt: &str) -> StatementStatus {
    match stmnt {
             INSERT_STMNT => { 
                println!("Insert stmnt");
            },
             SELECT_STMNT => { 
                println!("Select stmnt");
            },
            _ => { 
                println!("Unknown statement: {}", stmnt);
                return StatementStatus::Unknown;
            }
    }
    return StatementStatus::Success;
}

