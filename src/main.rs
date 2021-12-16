use std::io::Write;
fn main() {
    let mut input = String::new();
    loop {
        print!("db> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You introduced: {}", input);
        input.clear();
    }
}
