use std::env;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No command passed");
        return;
    }

    let command = args[1].clone();
    let name = "Victoria";
    let status = "100%";

    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    }
    else if command == "status" {
        println!("Status is {}", status);
    }
    else {
        println!("No command passed");
    }
}