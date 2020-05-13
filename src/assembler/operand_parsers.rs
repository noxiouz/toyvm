use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::sequence::{preceded, tuple};

use crate::assembler::Token;

pub fn integer_operand_parser(input: &str) -> nom::IResult<&str, Token> {
    preceded(tag("#"), digit1)(input).and_then(|(rest, value)| match value.parse::<i32>() {
        Ok(num) => Ok((rest, Token::IntegerOperand(num))),
        Err(_) => Err(nom::Err::Error((input, nom::error::ErrorKind::Digit))),
    })
}

pub fn register_parser(input: &str) -> nom::IResult<&str, Token> {
    preceded(tag("$"), digit1)(input).and_then(|(rest, value)| match value.parse::<u8>() {
        Ok(num) => Ok((rest, Token::Register(num))),
        Err(_) => Err(nom::Err::Error((input, nom::error::ErrorKind::Digit))),
    })
}

pub fn string_operand_parser(input: &str) -> nom::IResult<&str, Token> {
    tuple((tag("'"), take_until("'"), tag("'")))(input)
        .map(|(rest, (_, value, _))| (rest, Token::StringOperand(value.to_string())))
}

pub fn any_operand_parser(input: &str) -> nom::IResult<&str, Token> {
    alt((
        integer_operand_parser,
        register_parser,
        string_operand_parser,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register_parser("$0");
        assert_eq!(result.is_ok(), true);
        let result = register_parser("0");
        assert_eq!(result.is_ok(), false);
        let result = register_parser("$a");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_integer_operand() {
        // Test a valid integer operand
        let result = integer_operand_parser("#10");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IntegerOperand(10));

        // Test an invalid one (missing the #)
        let result = integer_operand_parser("10");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_string_operand() {
        // Test a valid integer operand
        let result = string_operand_parser("'test_string_ABZ'");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::StringOperand("test_string_ABZ".to_string()));

        // Test an invalid one (missing the #)
        let result = string_operand_parser("10");
        assert_eq!(result.is_ok(), false);
    }
}
