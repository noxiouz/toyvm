use crate::instructions::Opcode;
use std::result::Result;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
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
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
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
}
