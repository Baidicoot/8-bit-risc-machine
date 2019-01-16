extern crate risc_vm;

use std::{fs, process, env};

fn main() {
    let args: Vec<String> = vec![String::from("example.vmc")];
    let discs: Vec<String> = args
        .iter()
        .map(|path| fs::read_to_string(&path)
            .unwrap_or_else(|err| {
                println!("ERROR parsing arguments: {}", err);
                process::exit(1);
            })
        )
        .collect();
    risc_vm::run(discs).unwrap_or_else(|err| {
        println!("Application ERROR: {}", err);
        process::exit(2);
    });
}
