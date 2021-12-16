use std::io::Write;

const EXIT_CMD: &str = ".exit";

enum MetaCmdStatus {
        Success,
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
            _ => println!("Unknown command: {}", cmd)
    }
    return MetaCmdStatus::Success;
}
