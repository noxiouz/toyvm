use nom::character::complete::alpha1;
use nom::IResult;

use crate::assembler::Token;
use crate::instructions::Opcode;

pub fn opcode_parser(input: &str) -> IResult<&str, Token> {
    alpha1(input).map(|(rest, opcode)| (rest, Token::Op(Opcode::from(opcode))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_parser_valid_opcode() {
        let result = opcode_parser("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op(Opcode::LOAD));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_opcode_parser_valid_opcode_uppercase() {
        let result = opcode_parser("LOAD");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op(Opcode::LOAD));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_opcode_parser_invalid_opcode() {
        let result = opcode_parser("aold");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op(Opcode::IGL(0xFF)));
        assert_eq!(rest, "");
    }
}
