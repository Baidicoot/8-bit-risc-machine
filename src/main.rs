#![feature(duration_as_u128)]
#[macro_use] extern crate lazy_static;
extern crate risc_vm;
mod format;
mod compile;
use std::fs;
use std::time::Instant;

use std::{process, env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Argument ERROR: Not enough arguments supplied.");
        process::exit(1);
    }
    let command = &args[1];
    let extension = Path::new(&args[2]).extension().unwrap_or_else(|| {
        println!("Filetype ERROR: Could not infer filetype.");
        process::exit(2);
    });
    if command == "run" {
        if extension == "red" {
            let start = Instant::now();
            risc_vm::run(format::debug(&args[2])).unwrap_or_else(|err| {
                println!("Application ERROR: {}", err);
                process::exit(3);
            });
            println!("\nFIN: Program ran for {}ms.", start.elapsed().as_millis())
        } else if extension == "rex" {
            let start = Instant::now();
            risc_vm::run(format::bytes(&args[2])).unwrap_or_else(|err| {
                println!("Application ERROR: {}", err);
                process::exit(3);
            });
            println!("\nFIN: Program ran for {}ms.", start.elapsed().as_millis())
        } else {
            println!("Filetype ERROR: Did not recognise filetype.");
            process::exit(2);
        }
    } else if command == "compile" {
        if extension == "rasm" {
            if args.len() < 4 {
                println!("Argument ERROR: Missing location to compile to.");
                process::exit(5);
            } else {
                let location = &args[3];
                let extension = Path::new(&args[3]).extension().unwrap_or_else(|| {
                    println!("Filetype ERROR: Could not infer filetype.");
                    process::exit(2);
                });
                if extension == "rex" {
                    fs::write(location, compile::rasm(&args[2]).unwrap_or_else(|err| {
                        println!("Compilation ERROR: {}", err);
                        process::exit(7);
                    })).unwrap_or_else(|err| {
                        println!("Filesystem ERROR: {}", err);
                        process::exit(6);
                    });
                } else {
                    println!("Filetype ERROR: Did not recognise filetype.");
                    process::exit(2);
                }
            }
        }
    } else {
        println!("Command ERROR: {:?} is not a valid command", command);
        process::exit(4)
    }
}
