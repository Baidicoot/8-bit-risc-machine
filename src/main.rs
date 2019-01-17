extern crate risc_vm;
mod format;
//extern crate compile;

use std::{process, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if {args.len() < 3} {
        println!("Argument ERROR: Not enough arguments supplied.");
        process::exit(1);
    }
    let command = &args[1];
    if {command == "run"} {
        risc_vm::run(format::debug(&args[2])).unwrap_or_else(|err| {
            println!("Application ERROR: {}", err);
            process::exit(3);
        })
    } else {
        println!("Command ERROR: {:?} is not a valid command", command);
        process::exit(4)
    }
}
