use crate::assembler::instruction_parsers::{instruction, AssemblerInstruction};
use nom::multi::many1;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.instructions
            .iter()
            .map(|instruction| instruction.to_bytes())
            .flatten()
            .collect()
    }
}

pub fn program_parser(input: &str) -> nom::IResult<&str, Program> {
    let (input, instructions) = many1(instruction)(input)?;
    Ok((
        input,
        Program {
            instructions: instructions,
        },
    ))
}

#[test]
fn test_parse_program() {
    let result = program_parser("load $0 #100\n");
    assert_eq!(result.is_ok(), true);
    let (leftover, p) = result.unwrap();
    assert_eq!(leftover, "");
    assert_eq!(1, p.instructions.len());
}

#[test]
fn test_program_to_bytes() {
    let result = program_parser("load $0 #100\n");
    assert_eq!(result.is_ok(), true);
    let (_, program) = result.unwrap();
    let bytecode = program.to_bytes();
    assert_eq!(bytecode.len(), 4);
    println!("{:?}", bytecode);
}
