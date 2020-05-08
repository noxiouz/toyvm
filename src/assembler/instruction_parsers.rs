use nom::character::complete::{multispace0, space1};
use nom::sequence::{terminated, tuple};

use crate::assembler::opcode_parsers::opcode_parser;
use crate::assembler::operand_parsers::integer_operand_parser;
use crate::assembler::register_parsers::register_parser;
use crate::assembler::Token;
use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match &self.opcode {
            Token::Op { code } => results.push(u8::from(code)),
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
        return results;
    }

    // TODO: add From<Token> for u8
    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
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
fn args_reg_value(opcode: Token, input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = tuple((
        terminated(register_parser, space1),
        terminated(integer_operand_parser, multispace0),
    ));
    let (input, (register, operand)) = parser(input)?;
    let asm_instruction = AssemblerInstruction {
        opcode: opcode,
        operand1: Some(register),
        operand2: Some(operand),
        operand3: None,
    };
    Ok((input, asm_instruction))
}

// <$REGISTER> <$REGISTER> <$REGISTER>
// ADD $0 $1 $2\n
fn args_reg_reg_reg(opcode: Token, input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = tuple((
        terminated(register_parser, space1),
        terminated(register_parser, space1),
        terminated(register_parser, multispace0),
    ));
    let (input, (reg1, reg2, reg3)) = parser(input)?;
    let asm_instruction = AssemblerInstruction {
        opcode: opcode,
        operand1: Some(reg1),
        operand2: Some(reg2),
        operand3: Some(reg3),
    };
    Ok((input, asm_instruction))
}

// <$REGISTER> <$REGISTER>
// EQ $0 $1\n
fn args_reg_reg(opcode: Token, input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = tuple((
        terminated(register_parser, space1),
        terminated(register_parser, multispace0),
    ));
    let (input, (reg1, reg2)) = parser(input)?;
    let asm_instruction = AssemblerInstruction {
        opcode: opcode,
        operand1: Some(reg1),
        operand2: Some(reg2),
        operand3: None,
    };
    Ok((input, asm_instruction))
}

// <OPCODE> <$REGISTER>
// JMP $0 \n
fn args_reg(opcode: Token, input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let parser = terminated(register_parser, multispace0);
    let (input, reg1) = parser(input)?;
    let asm_instruction = AssemblerInstruction {
        opcode: opcode,
        operand1: Some(reg1),
        operand2: None,
        operand3: None,
    };
    Ok((input, asm_instruction))
}

// // <OPCODE>
// // HLT
fn args_none(opcode: Token, input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let asm_instruction = AssemblerInstruction {
        opcode: opcode,
        operand1: None,
        operand2: None,
        operand3: None,
    };
    Ok((input, asm_instruction))
}

pub fn instruction(input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let (input, opcode) = terminated(opcode_parser, multispace0)(input)?;
    let parser = match &opcode {
        Token::Op { code } => match code {
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
    parser(opcode, input)
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
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_reg_reg_reg() {
        let result = instruction("ADD $0 $1 $2\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 }),
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
                    opcode: Token::Op { code: Opcode::HLT },
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
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
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
        let result = instruction("JMP $9");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::JMP },
                    operand1: Some(Token::Register { reg_num: 9 }),
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}
