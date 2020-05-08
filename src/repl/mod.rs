use std;
use std::io;
use std::io::Write;

use crate::assembler::program_parsers::program_parser;
use crate::vm::VM;

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
                _ => match program_parser(buffer) {
                    Ok((_, result)) => {
                        let bytecode = result.to_bytes();
                        for byte in bytecode {
                            self.vm.add_byte(byte);
                        }
                        if let Err(e) = self.vm.run_once() {
                            println!("exit {}", e);
                            std::process::exit(1);
                        }
                    }
                    Err(_) => {
                        println!("Unable to parse input");
                        continue;
                    }
                },
            }
        }
    }
}
