#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,  // Halt
    LOAD, // Load
    // math
    ADD,
    MUL,
    SUB,
    DIV,
    // jumps
    JMP,
    JMPF,
    JMPB,
    JEQ,
    // equality
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    // utility
    IGL, // Illegal
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Opcode {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::JEQ,
            0xA => Opcode::EQ,
            0xB => Opcode::NEQ,
            0xC => Opcode::GT,
            0xD => Opcode::LT,
            0xE => Opcode::GTQ,
            0xF => Opcode::LTQ,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
