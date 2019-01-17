use std::{fs, process};

fn force_len(mut v: Vec<u8>, l: usize) -> Vec<u8> { //force len of vec
    if {v.len() == l} {v}
    else if {v.len() > l} {v[0..l].to_vec()}
    else {
        for i in 0..l-v.len() {
            v.push(0);
        }
        v
    }
}

fn force_disc(mut v: Vec<u8>) -> [u8; 256] {
    let mut arr = [0; 256];
    arr.copy_from_slice(&force_len(v, 256));
    arr
}

pub fn bytes(path: &String) -> Vec<[u8; 256]> { //read bytes from a file
    fs::read(path)
        .unwrap_or_else(|err| {
            println!("Argument ERROR: {}", err);
            process::exit(1);
        })
        .split(|b| (b == &0b11111111))
        .map(|d| {
            force_disc(d.to_vec())
        })
        .collect()
}

pub fn debug(path: &String) -> Vec<[u8; 256]> { //read a risc debug executable
    fn parse_byte(x: &str) -> u8 {
        u8::from_str_radix(x, 2).unwrap_or_else(|err| {
            println!("Format ERROR: Couldn't parse {:?} to byte.", x);
            process::exit(2);
        })
    }
    fs::read_to_string(path)
        .unwrap_or_else(|err| {
            println!("Argument ERROR: {}", err);
            process::exit(1);
        })
        .split(|d| (d == '-'))
        .map(|d| {
            force_disc(
                d.to_string()
                    .split(|d| (d == ' ') || (d == '\n') || (d == '\r'))
                    .filter(|d| (d.len() > 0))
                    .map(parse_byte)
                    .collect()
            )
        })
        .collect()
}
