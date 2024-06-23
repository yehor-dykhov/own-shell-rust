use std::{env, fs};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;
use std::process::{Command as ProcessCommand};

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

fn contains_executable_file_by_path<'a>(name: &'a str, paths: &Vec<&str>) -> Option<String> {
    for p in paths {
        let directory = fs::read_dir(p).unwrap();

        for file in directory {
            let file_path = file.unwrap();

            if file_path.file_name() == name {
                return Some(file_path.path().to_str().unwrap().to_owned());
            }
        }
    }

    return None;
}

fn main() {
    let path_val = env::var("PATH").unwrap_or("".to_owned());
    let paths: Vec<&str> = path_val.split(":").collect();

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
                let command = Command::from_str(arg_text).unwrap();

                match command {
                    Command::Unknown => {
                        let found_path = contains_executable_file_by_path(arg_text, &paths);

                        match found_path {
                            Some(p) => println!("{} is {}", arg_text, p),
                            None => println!("{}: not found", arg_text),
                        }
                    }
                    _ => println!("{} is a shell builtin", arg_text),
                }
            }
            Command::Unknown => {
                let found_path = contains_executable_file_by_path(command_text, &paths);

                if let Some(_) = found_path {
                    let _ = ProcessCommand::new(command_text).arg(arg_text).spawn().unwrap().wait();
                } else {
                    println!("{}: command not found", trimmed_input);
                }
            }
        }
    }
}
