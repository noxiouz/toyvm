use nom::character::complete::{multispace0, space1};
use nom::combinator::opt;
use nom::sequence::{terminated, tuple};

use crate::assembler::label_parsers::label_declaration_parser;
use crate::assembler::opcode_parsers::opcode_parser;
use crate::assembler::operand_parsers::{integer_operand_parser, register_parser};
use crate::assembler::Token;
use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub enum Action {
    Opcode(Token),
    Directive(Token),
}

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub label: Option<Token>,
    pub action: Action,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match &self.action {
            Action::Opcode(Token::Op(code)) => results.push(u8::from(code)),
            _ => panic!("Non-opcode found in opcode field"),
        };
        // TODO: make it nicer
        match (&self.operand1, &self.operand2, &self.operand3) {
            (Some(op1), Some(op2), Some(op3)) => {
                AssemblerInstruction::extract_operand(&op1, &mut results);
                AssemblerInstruction::extract_operand(&op2, &mut results);
                AssemblerInstruction::extract_operand(&op3, &mut results);
            }
            (Some(op1), Some(op2), None) => {
                AssemblerInstruction::extract_operand(&op1, &mut results);
                AssemblerInstruction::extract_operand(&op2, &mut results);
            }
            (Some(op1), None, None) => {
                AssemblerInstruction::extract_operand(&op1, &mut results);
            }
            (None, None, None) => (),
            _ => panic!("malformed AssemblerInstruction"),
        }
        results
    }

    // TODO: add From<Token> for u8
    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register(reg_num) => {
                results.push(*reg_num);
            }
            Token::IntegerOperand(value) => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                panic!("Opcode found in operand field");
            }
        };
    }
}

// <$REGISTER> <#VALUE>
// LOAD $0 #100
fn args_reg_value(
    mut asm_instruction: AssemblerInstruction,
    input: &str,
) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = tuple((
        terminated(register_parser, space1),
        terminated(integer_operand_parser, multispace0),
    ));
    let (input, (register, operand)) = parser(input)?;
    asm_instruction.operand1.replace(register);
    asm_instruction.operand2.replace(operand);
    Ok((input, asm_instruction))
}

// <$REGISTER> <$REGISTER> <$REGISTER>
// ADD $0 $1 $2\n
fn args_reg_reg_reg(
    mut asm_instruction: AssemblerInstruction,
    input: &str,
) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = tuple((
        terminated(register_parser, space1),
        terminated(register_parser, space1),
        terminated(register_parser, multispace0),
    ));
    let (input, (reg1, reg2, reg3)) = parser(input)?;
    asm_instruction.operand1.replace(reg1);
    asm_instruction.operand2.replace(reg2);
    asm_instruction.operand3.replace(reg3);
    Ok((input, asm_instruction))
}

// <$REGISTER> <$REGISTER>
// EQ $0 $1\n
fn args_reg_reg(
    mut asm_instruction: AssemblerInstruction,
    input: &str,
) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = tuple((
        terminated(register_parser, space1),
        terminated(register_parser, multispace0),
    ));
    let (input, (reg1, reg2)) = parser(input)?;
    asm_instruction.operand1.replace(reg1);
    asm_instruction.operand2.replace(reg2);
    Ok((input, asm_instruction))
}

// <OPCODE> <$REGISTER>
// JMP $0 \n
fn args_reg(
    mut asm_instruction: AssemblerInstruction,
    input: &str,
) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = terminated(register_parser, multispace0);
    let (input, reg1) = parser(input)?;
    asm_instruction.operand1.replace(reg1);
    Ok((input, asm_instruction))
}

// // <OPCODE>
// // HLT
fn args_none(
    asm_instruction: AssemblerInstruction,
    input: &str,
) -> nom::IResult<&str, AssemblerInstruction> {
    Ok((input, asm_instruction))
}

pub fn instruction(input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let (input, label) = terminated(opt(label_declaration_parser), multispace0)(input)?;
    let (input, opcode) = terminated(opcode_parser, multispace0)(input)?;
    let parser = match &opcode {
        Token::Op(code) => match code {
            Opcode::HLT => args_none,
            Opcode::LOAD => args_reg_value,
            Opcode::ADD => args_reg_reg_reg,
            Opcode::SUB => args_reg_reg_reg,
            Opcode::MUL => args_reg_reg_reg,
            Opcode::DIV => args_reg_reg_reg,
            Opcode::JMP => args_reg,
            Opcode::JMPF => args_reg,
            Opcode::JMPB => args_reg,
            Opcode::JEQ => args_reg,
            Opcode::EQ => args_reg_reg,
            Opcode::NEQ => args_reg_reg,
            Opcode::GT => args_reg_reg,
            Opcode::LT => args_reg_reg,
            Opcode::GTQ => args_reg_reg,
            Opcode::LTQ => args_reg_reg,
            Opcode::IGL(_) => args_none,
        },
        _ => panic!("non Opcode output from opcode parser"),
    };
    let asm_instruction = AssemblerInstruction {
        label,
        action: Action::Opcode(opcode),
        operand1: None,
        operand2: None,
        operand3: None,
    };
    parser(asm_instruction, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::Opcode;

    #[test]
    fn test_parse_instruction_reg_value() {
        let result = instruction("load $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    label: None,
                    action: Action::Opcode(Token::Op(Opcode::LOAD)),
                    operand1: Some(Token::Register(0)),
                    operand2: Some(Token::IntegerOperand(100)),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_reg_reg_reg() {
        let result = instruction("add:  ADD $0 $1 $2\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    label: Some(Token::LabelDeclaration("add".to_string())),
                    action: Action::Opcode(Token::Op(Opcode::ADD)),
                    operand1: Some(Token::Register(0)),
                    operand2: Some(Token::Register(1)),
                    operand3: Some(Token::Register(2)),
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_no_args() {
        let result = instruction("HLT\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    label: None,
                    action: Action::Opcode(Token::Op(Opcode::HLT)),
                    operand1: None,
                    operand2: None,
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_reg_value_upper_case() {
        let result = instruction("LOAD $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    label: None,
                    action: Action::Opcode(Token::Op(Opcode::LOAD)),
                    operand1: Some(Token::Register(0)),
                    operand2: Some(Token::IntegerOperand(100)),
                    operand3: None
                }
            ))
        );
    }

    // TODO: fix using per-opcode parser dispatch
    // as opcode knows args format
    #[test]
    fn test_error_parse_instruction_reg_value_upper_case() {
        let result = instruction("LOAD $0 100\n");
        assert!(result.is_err())
    }

    #[test]
    fn test_parse_instruction_reg() {
        let result = instruction("jump: JMP $9");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    label: Some(Token::LabelDeclaration("jump".to_string())),
                    action: Action::Opcode(Token::Op(Opcode::JMP)),
                    operand1: Some(Token::Register(9)),
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}
