use crate::instructions::Opcode;
use std::result::Result;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), u8> {
        loop {
            if self.pc >= self.program.len() {
                println!("pc overflow");
                break Err(1);
            }

            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLT encountered");
                    break Ok(());
                }
                Opcode::IGL => {
                    println!("IGL encountered");
                    break Err(2);
                }
                Opcode::LOAD => {
                    let register = self.next_8_bits() as usize;
                    let number = self.next_16_bits() as u16;
                    self.registers[register] = number as i32;
                    continue;
                }
                Opcode::ADD => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 + register2;
                }
                Opcode::MUL => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    // TODO: handle overflow
                    self.registers[self.next_8_bits() as usize] = register1 * register2;
                }
                Opcode::SUB => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 - register2;
                }
                Opcode::DIV => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 / register2;
                    self.remainder = (register1 % register2) as u32;
                }
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        assert_eq!(test_vm.run(), Ok(()));
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        assert_eq!(test_vm.run(), Err(2));
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244, 0];
        assert_eq!(test_vm.run(), Ok(()));
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_load_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244, 1, 1, 1, 245, 2, 0, 1, 4, 0];
        assert_eq!(test_vm.run(), Ok(()));
        assert_eq!(test_vm.registers[4], 1001);
    }

    #[test]
    fn test_load_sub() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244, 1, 1, 1, 245, 3, 1, 0, 4, 0];
        assert_eq!(test_vm.run(), Ok(()));
        assert_eq!(test_vm.registers[4], 1);
    }

    #[test]
    fn test_load_mul() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244, 1, 1, 1, 245, 4, 1, 0, 5, 0];
        assert_eq!(test_vm.run(), Ok(()));
        assert_eq!(test_vm.registers[5], 250500);
    }

    #[test]
    fn test_load_div() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 246, 1, 1, 1, 244, 5, 0, 1, 5, 0];
        assert_eq!(test_vm.run(), Ok(()));
        assert_eq!(test_vm.registers[5], 1);
        assert_eq!(test_vm.remainder, 2);
    }
}
