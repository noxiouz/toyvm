use nom;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::preceded;

use crate::assembler::Token;

pub fn integer_operand_parser(input: &str) -> nom::IResult<&str, Token> {
    preceded(tag("#"), digit1)(input).and_then(|(rest, value)| match value.parse::<i32>() {
        Ok(num) => Ok((rest, Token::IntegerOperand { value: num })),
        Err(_) => Err(nom::Err::Error((input, nom::error::ErrorKind::Digit))),
    })
}

#[test]
fn test_parse_integer_operand() {
    // Test a valid integer operand
    let result = integer_operand_parser("#10");
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(value, Token::IntegerOperand { value: 10 });

    // Test an invalid one (missing the #)
    let result = integer_operand_parser("10");
    assert_eq!(result.is_ok(), false);
}
