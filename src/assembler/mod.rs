pub mod directive_parsers;
pub mod instruction_parsers;
pub mod label_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parsers;

use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    // actions
    Directive(String),
    Op(Opcode),
    // operands
    IntegerOperand(i32),
    StringOperand(String),
    Register(u8),
    // labels
    LabelDeclaration(String),
    LabelUsage(String),
}
