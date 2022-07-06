use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::io::Write;
use std::collections::HashMap;

pub fn bs() {
    let args: Vec<String> = env::args().collect();
    if &args.len() > &1 && Path::new(&args[2]).exists() {
        let contents = fs::read_to_string(&args[2])
            .expect("Something went wrong reading the file");
        let formatted = Regex::new(r"\r?\n").unwrap().replace_all(&contents, ";").to_lowercase();
        let commands = formatted.split(';');
        let mut aliases: HashMap<String, u16> = HashMap::new();
        let mut bytes: Vec<u8> = Vec::new();
        for command in commands {
            if command.starts_with('#') {continue}
            let mut segs = command.trim().split(' ');
            match segs.next().unwrap() {
                "alias" => {
                    let label = segs.next().unwrap();
                    let mut address = segs.next().unwrap();
                    let radix: u32 = if address.starts_with("0x") { 16 } else { 10 };
                    if radix == 16 { address = &address[2..address.len()] }
                    let addr = u16::from_str_radix(address, radix).unwrap();
                    aliases.insert(label.to_string(), addr);
                }
                "push" => {
                    for mut byte in segs {
                        bytes.push(3);
                        if byte.starts_with("$") {
                            match aliases.get(&byte[1..byte.len()]) {
                                Some(addr) => {
                                    bytes.push((addr >> 8) as u8);
                                    bytes.push(3);
                                    bytes.push(addr.to_owned() as u8);
                                },
                                None => panic!("{} is not a valid alias", byte)
                            }
                        } else {
                            let radix: u32 = if byte.starts_with("0x") { 16 } else { 10 };
                            if radix == 16 { byte = &byte[2..byte.len()] }
                            bytes.push(u8::from_str_radix(byte, radix).unwrap());
                        }
                    }
                }
                "store" => {
                    bytes.push(0);
                }
                "routine" => {
                    bytes.push(0x0b);
                    let mut byte = segs.next().unwrap();
                    if byte.starts_with("$") {
                        match aliases.get(&byte[1..byte.len()]) {
                            Some(addr) => {
                                bytes.push((addr >> 8) as u8);
                                bytes.push(addr.to_owned() as u8);
                            },
                            None => panic!("{} is not a valid alias", byte)
                        }
                    } else {
                        let radix: u32 = if byte.starts_with("0x") { 16 } else { 10 };
                        if radix == 16 { byte = &byte[2..byte.len()] }
                        let addr = u16::from_str_radix(byte, radix).unwrap();
                        bytes.push((addr >> 8) as u8);
                        bytes.push(addr as u8);
                    }
                }
                "duplicate" => {
                    bytes.push(0x0e);
                }
                "copy" => {
                    bytes.push(0x0d);
                }
                "get" => {
                    bytes.push(1);
                }
                "add" => {
                    bytes.push(6);
                }
                "write" => {
                    bytes.push(5);
                }
                "subtract" => {
                    bytes.push(7);
                }
                "if" => {
                    let boolean = segs.next().unwrap();
                    if boolean == "nonzero" { bytes.push(0x11) } else if boolean == "zero" { bytes.push(0x12) } else { panic!("Boolean must be nonzero or zero"); }
                    let mut byte = segs.next().unwrap();
                    let radix: u32 = if byte.starts_with("0x") { 16 } else { 10 };
                    if radix == 16 { byte = &byte[2..byte.len()] }
                    let addr = u16::from_str_radix(byte, radix).unwrap();
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "end" => {
                    bytes.push(0x10);
                }
                "call" => {
                    bytes.push(0x0c);
                }
                "swap" => {
                    bytes.push(0x0f);
                }
                "lessthan" => {
                    bytes.push(0x14);
                }
                "greaterthan" => {
                    bytes.push(0x13);
                }
                "pop" => {
                    bytes.push(4);
                }
                "print" => {
                    bytes.push(2);
                }
                "multiply" => {
                    bytes.push(8);
                }
                "divide" => {
                    bytes.push(9);
                }
                "modulo" => {
                    bytes.push(0x0a);
                }
                "sprite" => {
                    bytes.push(0x15);
                    for mut byte in segs {
                        let radix: u32 = if byte.starts_with("0x") { 16 } else { 10 };
                        if radix == 16 { byte = &byte[2..byte.len()] }
                        bytes.push(u8::from_str_radix(byte, radix).unwrap());
                    }
                    bytes.push(0xff);
                }
                "bitnot" => {
                    bytes.push(0x16);
                }
                "bitand" => {
                    bytes.push(0x17);
                }
                "bitor" => {
                    bytes.push(0x18);
                }
                "bitxor" => {
                    bytes.push(0x19);
                }
                "random" => {
                    bytes.push(0x1a);
                }
                "repeat" => {
                    bytes.push(0x1b);
                    let mut byte = segs.next().unwrap();
                    let radix: u32 = if byte.starts_with("0x") { 16 } else { 10 };
                    if radix == 16 { byte = &byte[2..byte.len()] }
                    let addr = u16::from_str_radix(byte, radix).unwrap();
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "beep" => {
                    bytes.push(0x1c);
                }
                "boop" => {
                    bytes.push(0x1e);
                }
                "noise" => {
                    bytes.push(0x1f);
                }
                "empty" => {
                    bytes.push(0x1d);
                }
                _ => {}
            }
        }
        let mut file = fs::File::create(format!("{}.flc", &args[2])).unwrap();
        file.write(bytes.as_slice()).unwrap();
    } else {
        println!("Please specify a filename to read, or ensure the file you provided exists");
    }
}
