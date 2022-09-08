use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use super::label::Label;
use super::util::hexadecimal;

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Simbolic(Label<'a>),
    Numeric(u16),
}

impl<'a> Operand<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            // numeric
            map(alt((preceded(tag("/"), hexadecimal),)), |value: u16| {
                Self::new_numeric(value)
            }),
            // simbolic
            map(Label::parse, |label| Self::new_simbolic(label)),
        ))(input)
    }

    pub fn new_numeric(value: u16) -> Self {
        Self::Numeric(value)
    }

    pub fn new_simbolic(label: Label<'a>) -> Self {
        Self::Simbolic(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_numeric() {
        assert_eq!(Operand::parse("/000F"), Ok(("", Operand::new_numeric(15))));
        assert_eq!(Operand::parse("/F"), Ok(("", Operand::new_numeric(15))));
    }

    #[test]
    fn should_parse_simbolic() {
        assert_eq!(
            Operand::parse("label"),
            Ok(("", Operand::new_simbolic(Label::new("label"))))
        );
        assert!(Operand::parse("1label").is_err());
    }
}
