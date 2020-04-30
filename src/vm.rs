use crate::instructions::Opcode;
use std::result::Result;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

#[derive(Debug)]
pub enum Step {
    Done,
    Continue,
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
            match self.run_once() {
                Err(err) => break Err(err),
                Ok(Step::Done) => break Ok(()),
                Ok(Step::Continue) => (),
            }
        }
    }

    pub fn run_once(&mut self) -> Result<Step, u8> {
        if self.pc >= self.program.len() {
            println!("pc overflow");
            return Err(1);
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                Ok(Step::Done)
            }
            Opcode::IGL => {
                println!("IGL encountered");
                Err(2)
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
                Ok(Step::Continue)
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                Ok(Step::Continue)
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                // TODO: handle overflow
                self.registers[self.next_8_bits() as usize] = register1 * register2;
                Ok(Step::Continue)
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
                Ok(Step::Continue)
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                Ok(Step::Continue)
            }
            Opcode::JMP => {
                let register = self.next_8_bits() as usize;
                let jump = self.registers[register];
                self.pc = jump as usize;
                Ok(Step::Continue)
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
                Ok(Step::Continue)
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= value;
                Ok(Step::Continue)
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

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        assert!(matches!(test_vm.run_once(), Ok(Step::Continue)));
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        assert!(matches!(test_vm.run_once(), Ok(Step::Continue)));
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        let jmp = 4;
        test_vm.program = vec![1, 0, 0, jmp, 8, 0, 0, 0];
        assert!(matches!(test_vm.run_once(), Ok(Step::Continue)));
        let before_jmp = test_vm.pc;
        assert!(matches!(test_vm.run_once(), Ok(Step::Continue)));
        // +2 comes from JMPB size
        assert_eq!(test_vm.pc, before_jmp - jmp as usize + 2);
    }
}
