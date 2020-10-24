use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, hex_digit1},
    combinator::{map, recognize},
    multi::many0,
    sequence::pair,
    IResult,
};

pub fn ident(code: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )(code)
}

// todo: 支持负数
pub fn integer(code: &str) -> IResult<&str, i64> {
    alt((
        map(pair(tag("0x"), hex_digit1), |(_, digits)| {
            i64::from_str_radix(digits, 16).unwrap()
        }),
        map(digit1, |digits| i64::from_str_radix(digits, 10).unwrap()),
    ))(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_ident() {
        let result = ident("a").unwrap().1;
        assert_eq!(result, "a".to_string());
        let result = ident("a_b_c").unwrap().1;
        assert_eq!(result, "a_b_c".to_string());
    }

    #[test]
    fn can_parse_integer() {
        let result = integer("0x40000000").unwrap().1;
        assert_eq!(result, 0x40000000);
        let result = integer("99").unwrap().1;
        assert_eq!(result, 99);
    }
}
