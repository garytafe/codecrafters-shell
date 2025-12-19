use std::env;
use std::io::{self, Write};

fn main() {
    let builtins = ["exit", "echo", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };

        match command {
            "exit" => break,
            "echo" => {
                let message: Vec<&str> = parts.collect();
                println!("{}", message.join(" "));
            }
            "type" => {
                let arg = parts.next().unwrap_or("");
                if builtins.contains(&arg) {
                    println!("{} is a shell builtin", arg);
                } else if let Some(path) = find_in_path(arg) {
                    println!("{} is {}", arg, path);
                } else {
                    println!("{}: not found", arg);
                }
            }
            _ => println!("{}: command not found", command),
        }
    }
}

fn find_in_path(cmd: &str) -> Option<String> {
    let path_var = env::var("PATH").ok()?;
    for path in env::split_paths(&path_var) {
        let full_path = path.join(cmd);
        if full_path.exists() {
            return Some(full_path.to_string_lossy().to_string());
        }
    }
    None
}