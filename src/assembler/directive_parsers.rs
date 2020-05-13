use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0};
use nom::combinator::opt;
use nom::sequence::{preceded, terminated, tuple};

use crate::assembler::instruction_parsers::{Action, AssemblerInstruction};
use crate::assembler::label_parsers::label_declaration_parser;
use crate::assembler::operand_parsers::any_operand_parser;
use crate::assembler::Token;

// .directive
fn directive_declaration_parse(input: &str) -> nom::IResult<&str, Token> {
    preceded(tag("."), alphanumeric1)(input)
        .map(|(input, directive)| (input, Token::Directive(directive.to_string())))
}

pub fn directive(input: &str) -> nom::IResult<&str, AssemblerInstruction> {
    let (input, label) = terminated(opt(label_declaration_parser), multispace0)(input)?;
    let (input, d) = terminated(directive_declaration_parse, multispace0)(input)?;
    let (input, (op1, op2, op3)) = tuple((
        terminated(opt(any_operand_parser), multispace0),
        terminated(opt(any_operand_parser), multispace0),
        terminated(opt(any_operand_parser), multispace0),
    ))(input)?;
    let asm_instruction = AssemblerInstruction {
        label,
        action: Action::Directive(d),
        operand1: op1,
        operand2: op2,
        operand3: op3,
    };
    Ok((input, asm_instruction))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_directive() {
        let result = directive_declaration_parse(".data");
        assert_eq!(result.is_ok(), true);
        let (_, directive) = result.unwrap();
        assert_eq!(directive, Token::Directive("data".to_string()))
    }

    #[test]
    fn test_string_directive() {
        let result = directive("test: .asciiz 'Hello'");
        assert_eq!(result.is_ok(), true);
        let (_, directive) = result.unwrap();

        // Yes, this is the what the result should be
        let correct_instruction = AssemblerInstruction {
            label: Some(Token::LabelDeclaration("test".to_string())),
            action: Action::Directive(Token::Directive("asciiz".to_string())),
            operand1: Some(Token::StringOperand("Hello".to_string())),
            operand2: None,
            operand3: None,
        };

        assert_eq!(directive, correct_instruction);
    }
}
