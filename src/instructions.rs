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
    IGL(u8), // Illegal
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
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
            _ => Opcode::IGL(v),
        }
    }
}

impl From<&Opcode> for u8 {
    fn from(op: &Opcode) -> Self {
        match op {
            Opcode::HLT => 0,
            Opcode::LOAD => 1,
            Opcode::ADD => 2,
            Opcode::SUB => 3,
            Opcode::MUL => 4,
            Opcode::DIV => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::JEQ => 9,
            Opcode::EQ => 0xA,
            Opcode::NEQ => 0xB,
            Opcode::GT => 0xC,
            Opcode::LT => 0xD,
            Opcode::GTQ => 0xE,
            Opcode::LTQ => 0xF,
            Opcode::IGL(_) => 0xFF,
        }
    }
}

impl<'a> From<&str> for Opcode {
    fn from(opcode: &str) -> Self {
        match opcode.to_uppercase().as_str() {
            "HLT" => Opcode::HLT,
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JMPB" => Opcode::JMPB,
            "JEQ" => Opcode::JEQ,
            "EQ" => Opcode::EQ,
            "NEQ" => Opcode::NEQ,
            "GT" => Opcode::GT,
            "LT" => Opcode::LT,
            "GTQ" => Opcode::GTQ,
            "LTQ" => Opcode::LTQ,
            _ => Opcode::IGL(0xFF),
        }
    }
}

// #[derive(Debug, PartialEq)]
// pub struct Instruction {
//     opcode: Opcode,
// }

// impl Instruction {
//     fn new(opcode: Opcode) -> Instruction {
//         Instruction { opcode: opcode }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_hlt() {
//         let opcode = Opcode::HLT;
//         assert_eq!(opcode, Opcode::HLT);
//     }

//     #[test]
//     fn test_create_instruction() {
//         let instruction = Instruction::new(Opcode::HLT);
//         assert_eq!(instruction.opcode, Opcode::HLT);
//     }
// }
