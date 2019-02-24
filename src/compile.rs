extern crate regex;
extern crate risc_vm;
use risc_vm::{DISCS, REGISTERS};
use regex::{RegexBuilder, Regex};
use std::collections::HashMap;

fn reg(s: &str) -> Regex {
    RegexBuilder::new(s)
        .multi_line(true)
        .build()
        .unwrap()
}

lazy_static!{
    static ref INS: Regex = reg(r"([A-Z]+) ([\w:@ ]+)");

    static ref DVV: Regex = reg(r":(\w+) (\w+) (\w+)");   //
    static ref DVR: Regex = reg(r":(\w+) (\w+) @(\w+)");  // Block
    static ref DRV: Regex = reg(r":(\w+) @(\w+) (\w+)");  //  One
    static ref DRR: Regex = reg(r":(\w+) @(\w+) @(\w+)"); //

    static ref RVV: Regex = reg(r":@(\w+) (\w+) (\w+)");   //
    static ref RVR: Regex = reg(r":@(\w+) (\w+) @(\w+)");  // Block
    static ref RRV: Regex = reg(r":@(\w+) @(\w+) (\w+)");  //  Two
    static ref RRR: Regex = reg(r":@(\w+) @(\w+) @(\w+)"); //

    static ref RV: Regex = reg(r"@(\w+) (\w+)");  // Block
    static ref RR: Regex = reg(r"@(\w+) @(\w+)"); // Three

    static ref VR: Regex = reg(r"(\w+) @(\w+)"); // Block Four

    static ref DV: Regex = reg(r"(\w+) :(\w+)");  // Block
    static ref DR: Regex = reg(r"(\w+) :(\w+)"); //  Five
}

enum Operand {
    DVV(u8, u8, u8),
    DVR(u8, u8, u8),
    DRV(u8, u8, u8),
    DRR(u8, u8, u8),

    RVV(u8, u8, u8),
    RVR(u8, u8, u8),
    RRV(u8, u8, u8),
    RRR(u8, u8, u8),

    DV(u8, u8),
    DR(u8, u8),

    RV(u8, u8),
    RR(u8, u8),

    VR(u8, u8),
}

impl Operand {
    pub fn check(self) -> Result<Operand, String> {
        use self::Operand::*;

        match self {
            DVV(a, b, c) => {
                if (a as usize) < DISCS {
                    Ok(self)
                } else {
                    Err(format!(":{}, {}, or {} are too big", a, b, c))
                }
            },
            DVR(a, b, c) => {
                if (a as usize) < DISCS && (c as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!(":{}, {}, or @{} are too big", a, b, c))
                }
            },
            DRV(a, b, c) => {
                if (a as usize) < DISCS && (b as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!(":{}, @{}, or {} are too big", a, b, c))
                }
            },
            DRR(a, b, c) => {
                if (a as usize) < DISCS && (b as usize) < REGISTERS && (c as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!(":{}, @{}, or @{} are too big", a, b, c))
                }
            },

            RVV(a, b, c) => {
                if (a as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!(":{}, {}, or {} are too big", a, b, c))
                }
            },
            RVR(a, b, c) => {
                if (a as usize) < REGISTERS && (c as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!(":{}, {}, or @{} are too big", a, b, c))
                }
            },
            RRV(a, b, c) => {
                if (a as usize) < REGISTERS && (b as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!(":{}, @{}, or {} are too big", a, b, c))
                }
            },
            RRR(a, b, c) => {
                if (a as usize) < REGISTERS && (b as usize) < REGISTERS && (c as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!(":{}, @{}, or @{} are too big", a, b, c))
                }
            },

            RV(a, b) => {
                if (a as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!("@{} or {} are too big", a, b))
                }
            },
            RR(a, b) =>{
                if (a as usize) < REGISTERS && (b as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!("@{} or {} are too big", a, b))
                }
            },

            VR(a, b) =>{
                if (b as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!("@{} or {} are too big", a, b))
                }
            },

            DV(a, b) => {
                if (a as usize) < DISCS {
                    Ok(self)
                } else {
                    Err(format!("@{} or {} are too big", a, b))
                }
            },
            DR(a, b) => {
                if (a as usize) < DISCS && (b as usize) < REGISTERS {
                    Ok(self)
                } else {
                    Err(format!("@{} or {} are too big", a, b))
                }
            },
        }
    }

    pub fn from_vec(v: &[u8], code: &str) -> Option<Operand> {
        use self::Operand::*;

        if v.len() < 2 {
            return None;
        }

        Some(match code {
            "RV" => RV(v[0], v[1]),
            "RR" => RR(v[0], v[1]),

            "VR" => VR(v[0], v[1]),

            "DV" => DV(v[0], v[1]),
            "DR" => DR(v[0], v[1]),

            _ => return Operand::from_vec_internal(v, code),
        })
    }

    fn from_vec_internal(v: &[u8], code: &str) -> Option<Operand> {
        use self::Operand::*;

        if v.len() < 3 {
            return None;
        }

        match code {
            "DVV" => Some(DVV(v[0], v[1], v[2])),
            "DVR" => Some(DVR(v[0], v[1], v[2])),
            "DRV" => Some(DRV(v[0], v[1], v[2])),
            "DRR" => Some(DRR(v[0], v[1], v[2])),

            "RVV" => Some(RVV(v[0], v[1], v[2])),
            "RVR" => Some(RVR(v[0], v[1], v[2])),
            "RRV" => Some(RRV(v[0], v[1], v[2])),
            "RRR" => Some(RRR(v[0], v[1], v[2])),
            _ => None,
        }
    }

    fn parse_captures(s: &str, code: &str) -> Option<Operand> {
        let cap = match code {
            "DVV" => DVV.captures(s),
            "DVR" => DVR.captures(s),
            "DRV" => DRV.captures(s),
            "DRR" => DRR.captures(s),

            "RVV" => RVV.captures(s),
            "RVR" => RVR.captures(s),
            "RRV" => RRV.captures(s),
            "RRR" => RRR.captures(s),

            "RV" => RV.captures(s),
            "RR" => RR.captures(s),

            "DV" => DV.captures(s),
            "DR" => DR.captures(s),

            "VR" => VR.captures(s),
            _ => return None,
        };

        match cap {
            Some(x) => {
                let args: Vec<Option<u8>> = x.iter()
                    .map(|i| {
                        let val = i.unwrap().as_str();

                        match val.parse::<u8>() {
                            Ok(x) => Some(x),
                            Err(_) => {
                                if val.len() == 1 {
                                    Some(val.chars().next().unwrap() as u8)
                                } else {
                                    None
                                }
                            }
                        }
                    })
                    .collect();

                let mut safe = vec![];
                
                for i in args[1..].iter() {
                    match i {
                        Some(x) => safe.push(*x),
                        None => return None,
                    }
                }

                Operand::from_vec(&safe, code)
            },
            None => None,
        }
    }

    pub fn from_codes(s: &str, codes: &[&str]) -> Option<Operand> {
        for code in codes.iter() {
            if let Some(x) = Operand::parse_captures(&s, code) {
                return Some(x);
            } else {
                continue;
            }
        }

        None
    }
}

fn op_codes(ins: &str) -> Option<Vec<&str>> {
    Some(match ins {
        "SAVE" => vec!["DVV", "DVR", "DRV", "DRR", "RVV", "RVR", "RRV", "RRR"],
        "LOAD" => vec!["DVR", "DRR", "RVR", "RRR"],
        "SET" => vec!["RV", "RR"],

        "ADD" => vec!["RV", "RR"],
        "SUB" => vec!["RV", "RR", "VR"],
        "OR" => vec!["RV", "RR"],
        "XOR" => vec!["RV", "RR"],
        "NOR" => vec!["RV", "RR"],
        "AND" => vec!["RV", "RR"],

        "GRT" => vec!["RV", "RR"],
        "LST" => vec!["RV", "RR"],
        "GREQT" => vec!["RV", "RR"],
        "LSEQT" => vec!["RV", "RR"],
        "EQL" => vec!["RV", "RR"],
        "NEQL" => vec!["RV", "RR"],

        "JZ" => vec!["DRV", "DRR", "RRV", "RRR"],
        "JNZ" => vec!["DRV", "DRR", "RRV", "RRR"],
        "JMP" => vec!["DR", "DV", "RR", "RV"],
        _ => return None,
    })
}

fn partial_code(ins: &str, code: &str) -> Result<u8, String> {
    Ok(match ins {
        "SAVE" => {
            match code {
                "DVV" => 0b00001000,
                "DVR" => 0b00010000,
                "DRV" => 0b00011000,
                "DRR" => 0b00100000,
                "RVV" => 0b00101000,
                "RVR" => 0b00110000,
                "RRV" => 0b00111000,
                "RRR" => 0b01000000,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "LOAD" => {
            match code {
                "DVR" => 0b01001000,
                "DRR" => 0b01010000,
                "RVR" => 0b01011000,
                "RRR" => 0b01100000,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "SET" => {
            match code {
                "RV" => 0b10000001,
                "RR" => 0b10000010,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },

        "ADD" => {
            match code {
                "RV" => 0b01101011,
                "RR" => 0b01101100,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "SUB" => {
            match code {
                "RV" => 0b01101000,
                "VR" => 0b01101001,
                "RR" => 0b01101010,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "OR" => {
            match code {
                "RV" => 0b01101101,
                "RR" => 0b01101110,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "XOR" => {
            match code {
                "RV" => 0b01101111,
                "RR" => 0b01110000,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "NOR" => {
            match code {
                "RV" => 0b01110001,
                "RR" => 0b01110010,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "AND" => {
            match code {
                "RV" => 0b01110011,
                "RR" => 0b01110100,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },

        "GRT" => {
            match code {
                "RV" => 0b01110101,
                "RR" => 0b01110110,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "LST" => {
            match code {
                "RV" => 0b01110111,
                "RR" => 0b01111000,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "GREQT" => {
            match code {
                "RV" => 0b01111001,
                "RR" => 0b01111010,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "LSEQT" => {
            match code {
                "RV" => 0b01111011,
                "RR" => 0b01111100,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "EQL" => {
            match code {
                "RV" => 0b01111101,
                "RR" => 0b01111110,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "NEQL" => {
            match code {
                "RV" => 0b01111111,
                "RR" => 0b10000000,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },

        "JZ" => {
            match code {
                "DRV" => 0b10001000,
                "DRR" => 0b10010000,
                "RRV" => 0b10011000,
                "RRR" => 0b10100000,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "JNZ" => {
            match code {
                "DRV" => 0b10101000,
                "DRR" => 0b10110000,
                "RRV" => 0b10111000,
                "RRR" => 0b11000000,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        "JMP" => {
            match code {
                "DV" => 0b10000011,
                "DR" => 0b10000100,
                "RV" => 0b10000101,
                "RR" => 0b10000110,
                _ => return Err(format!("operand type {} cannot be used with {}", code, ins)),
            }
        },
        _ => return Err(format!("did not recognise ins {}", ins))
    })
}

fn ins_code(ins: &str, ops: Operand) -> Result<[u8; 3], String> {
    use self::Operand::*;

    let (appendix, a, b, code) = match ops {
        DVV(a, b, c) => (Some(a), b, c, "DVV"),
        DVR(a, b, c) => (Some(a), b, c, "DVR"),
        DRV(a, b, c) => (Some(a), b, c, "DRV"),
        DRR(a, b, c) => (Some(a), b, c, "DRR"),

        RVV(a, b, c) => (Some(a), b, c, "RVV"),
        RVR(a, b, c) => (Some(a), b, c, "RVR"),
        RRV(a, b, c) => (Some(a), b, c, "RRV"),
        RRR(a, b, c) => (Some(a), b, c, "RRR"),

        DV(a, b) => (None, a, b, "DV"),
        DR(a, b) => (None, a, b, "DR"),

        RV(a, b) => (None, a, b, "RV"),
        RR(a, b) => (None, a, b, "RR"),

        VR(a, b) => (None, a, b, "VR"),
    };

    let ins_code = partial_code(ins, code)?;

    Ok([match appendix {
        Some(x) => ins_code+x,
        None => ins_code,
    }, a, b])
}

fn parse_instruction(ins: &str, op: &str) -> Result<[u8; 3], String> {
    let codes = match op_codes(ins) {
        Some(x) => x,
        None => return Err(format!("Could not recognise instruction {:?}", ins))
    };

    let ops = match Operand::from_codes(op, &codes) {
        Some(x) => x,
        None => return Err(format!("Could not compile operands {:?}", op)),
    };

    ins_code(ins, ops.check()?)
}

fn line(ln: &str) -> Result<[u8; 3], String> {
    if ln == "END" {
        Ok([0, 0, 0])
    } else {
        match INS.captures(ln) {
            Some(x) => parse_instruction(&x[1], &x[2]),
            None => Err(format!("Could not recognise instruction {:?}", ln))
        }
    }
}

pub fn rasm(path: &str) -> Result<Vec<u8>, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(x) => x,
        Err(x) => return Err(format!("{}", x)),
    };

    let discs: Vec<Vec<&str>> = content.split("---")
        .map(|y| {
            y.lines()
            .filter(|x| {
                x != &""
            })
            .collect()
        })
        .collect();

    let mut program = vec![];

    for (disc_index, d) in discs.iter().enumerate() {
        let mut disc = vec![];
        for (ins_index, i) in d.iter().enumerate() {
            let iterator = match line(i) {
                Ok(x) => x,
                Err(err) => return Err(format!("Errored at `{}` on disc {} at line {}.", err, disc_index, ins_index)),
            };

            disc.extend(iterator.iter());
        }
        program.append(&mut disc);
        program.push(0b11111111);
    }

    Ok(program)
}