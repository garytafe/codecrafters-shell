use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();   
        let text: Vec<&str> = command.split_whitespace().collect(); 
        let program = text[0];
        if program == "exit" {
            break;
        }
        if program == "echo" {
            let output = &command[5..]; 
            println!("{}", output.trim());
            continue;
        }
        println!("{}: command not found", command.trim())
    }
}
