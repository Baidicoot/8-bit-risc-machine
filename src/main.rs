extern crate risc_vm;
mod format;
//extern crate compile;

use std::{process, env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if {args.len() < 3} {
        println!("Argument ERROR: Not enough arguments supplied.");
        process::exit(1);
    }
    let command = &args[1];
    let extension = Path::new(&args[2]).extension().unwrap_or_else(|| {
        println!("Filetype ERROR: Could not infer filetype.");
        process::exit(2);
    });
    if {command == "run"} {
        if {extension == "red"} {
            risc_vm::run(format::debug(&args[2])).unwrap_or_else(|err| {
                println!("Application ERROR: {}", err);
                process::exit(3);
            });
        } else if {extension == "rex"} {
            risc_vm::run(format::bytes(&args[2])).unwrap_or_else(|err| {
                println!("Application ERROR: {}", err);
                process::exit(3);
            });
        } else {
            println!("Filetype ERROR: Did not recognise filetype.");
            process::exit(2);
        }
    } else {
        println!("Command ERROR: {:?} is not a valid command", command);
        process::exit(4)
    }
}
