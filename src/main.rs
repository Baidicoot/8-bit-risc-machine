use std::fs;

pub struct Config {
    path: String
}

impl Config {
    pub fn from_args(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let path = match args.next() {
            Some(i) => i,
            None => return Err("Didn't find a path argument."),
        };

         Ok(Config { path })
    }
}

fn main() {
    
}
