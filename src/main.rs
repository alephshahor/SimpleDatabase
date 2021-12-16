use std::io::Write;
const EXIT_CMD: &str = ".exit";

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
            if process_meta_cmd(processed_input) {
                break;
            }
        }

        input.clear();
    }
}

fn process_meta_cmd(cmd: &str) -> bool {
    match cmd {
            EXIT_CMD => { 
                println!("Exiting the program");
                return true;
            },
            _ => println!("Unknown command: {}", cmd)
    }
    return false;
}
