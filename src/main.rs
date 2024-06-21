#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        if trimmed_input == "exit 0" {
            break;
        } else if trimmed_input.contains("echo") {
            let echo_text: Vec<&str> = input.split("echo").collect();
            println!("{}", echo_text[1].trim());
        } else {
            println!("{}: command not found", trimmed_input);
        }
    }
}
