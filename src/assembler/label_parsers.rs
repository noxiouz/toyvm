use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::sequence::{preceded, terminated};

use crate::assembler::Token;

// label:
pub fn label_declaration_parser(input: &str) -> nom::IResult<&str, Token> {
    terminated(alphanumeric1, tag(":"))(input)
        .map(|(input, label)| (input, Token::LabelDeclaration(label.to_string())))
}

// @label
pub fn label_usage_parser(input: &str) -> nom::IResult<&str, Token> {
    preceded(tag("@"), alphanumeric1)(input)
        .map(|(input, label)| (input, Token::LabelUsage(label.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        let result = label_declaration_parser("test:");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelDeclaration("test".to_string()));
        let result = label_declaration_parser("test");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage_parser("@test");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelUsage("test".to_string()));
        let result = label_usage_parser("test");
        assert_eq!(result.is_ok(), false);
    }
}
