use nom;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::preceded;

use crate::assembler::Token;

pub fn register_parser(input: &str) -> nom::IResult<&str, Token> {
    preceded(tag("$"), digit1)(input).and_then(|(rest, value)| match value.parse::<u8>() {
        Ok(num) => Ok((rest, Token::Register { reg_num: num })),
        Err(_) => Err(nom::Err::Error((input, nom::error::ErrorKind::Digit))),
    })
}

#[test]
fn test_parse_register() {
    let result = register_parser("$0");
    assert_eq!(result.is_ok(), true);
    let result = register_parser("0");
    assert_eq!(result.is_ok(), false);
    let result = register_parser("$a");
    assert_eq!(result.is_ok(), false);
}
