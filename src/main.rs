use std::io::Write;
fn main() {
    let mut input = String::new();
    let EXIT_CMD = "exit";

    loop {
        print!("db> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // TODO: Handle defer error (if possible)
        match &input.trim()[..] {
            x if x == EXIT_CMD => { 
                println!("Exiting the program");
                break;
            },
            _ => println!("Unknown command: {}", input)
        }
        input.clear();
    }
}
