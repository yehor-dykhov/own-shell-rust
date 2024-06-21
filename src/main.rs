#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

enum Command {
    Exit,
    Echo,
    Type,
    Unknown,
}

// impl Command {
//     fn as_str(&self) -> &'static str {
//         match self {
//             Command::Exit => "exit",
//             Command::Echo => "echo",
//             Command::Type => "type",
//             _ => "",
//         }
//     }
// }

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s {
            "echo" => Ok(Command::Echo),
            "exit" => Ok(Command::Exit),
            "type" => Ok(Command::Type),
            _ => Ok(Command::Unknown),
        }
    }
}

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
        let input_values: Vec<&str> = input.split(" ").collect();
        let command_text = input_values[0].trim();
        let arg_text = if input_values.len() > 1 {
            input_values[1].trim()
        } else {
            ""
        };
        let command = Command::from_str(command_text).unwrap();

        match command {
            Command::Echo => {
                let arg: Vec<&str> = input.split("echo").collect();
                println!("{}", arg[1].trim());
            }
            Command::Exit => {
                break;
            }
            Command::Type => {
                // let args: Vec<&str> = input.split("type").collect();
                // let arg = args[1].trim();
                let command = Command::from_str(arg_text).unwrap();

                match command {
                    Command::Unknown => println!("{}: not found", arg_text),
                    _ => println!("{} is a shell builtin", arg_text),
                }
            }
            Command::Unknown => {
                println!("{}: command not found", trimmed_input);
            }
        }

        // if trimmed_input.contains("exit") {
        //     break;
        // } else if trimmed_input.contains("echo") {
        //     let echo_text: Vec<&str> = input.split("echo").collect();
        //     println!("{}", echo_text[1].trim());
        // } else {
        //     println!("{}: command not found", trimmed_input);
        // }
    }
}
