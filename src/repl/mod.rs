use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    vm: VM,

    commands_buffer: Vec<String>,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            vm: VM::new(),
            commands_buffer: vec![],
        }
    }

    pub fn run(&mut self) -> () {
        println!("welcome");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">>>>> ");
            io::stdout().flush().expect("failed to flush stdout");

            stdin.read_line(&mut buffer).expect("failed to read stdin");
            self.commands_buffer.push(buffer.trim().to_string());
            let buffer = buffer.trim();
            match buffer {
                ".quit" => {
                    println!("exit");
                    std::process::exit(0);
                }
                ".history" => {
                    for cmd in &self.commands_buffer {
                        println!("{}", cmd)
                    }
                }
                ".program" => {
                    self.vm
                        .program
                        .iter()
                        .for_each(|&instruction| println!("{:X?}", instruction));
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }
                _ => {
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte)
                            }
                            self.vm.run_once();
                        },
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.")
                        }
                    }
                }
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
