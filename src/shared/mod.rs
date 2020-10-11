pub mod data_type {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::{map, recognize};
    use nom::sequence::pair;
    use nom::IResult;
    use std::fmt;
    use std::fmt::Display;
    use std::str::FromStr;
    use sum_type::_core::fmt::Formatter;

    #[derive(Debug, Eq, PartialEq, Clone, Hash)]
    pub struct Integer {
        pub signed: bool,
        pub width: usize,
    }

    #[derive(Debug, Eq, PartialEq, Clone, Hash)]
    pub enum Type {
        Integer(Integer),
        Address,
    }

    impl Display for Type {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Type::Integer(i) => i.fmt(f),
                Type::Address => write!(f, "address"),
            }
        }
    }

    impl From<Integer> for Type {
        fn from(integer: Integer) -> Self {
            Type::Integer(integer)
        }
    }

    pub fn parse_integer(code: &str) -> IResult<&str, Integer> {
        alt((
            map(pair(tag("i"), digit1), |(_, width_str)| Integer {
                signed: true,
                width: usize::from_str(width_str).unwrap(),
            }),
            map(pair(tag("u"), digit1), |(_, width_str)| Integer {
                signed: false,
                width: usize::from_str(width_str).unwrap(),
            }),
        ))(code)
    }

    pub fn parse_type(code: &str) -> IResult<&str, Type> {
        alt((
            map(
                alt((recognize(pair(parse_integer, tag("*"))), tag("address"))),
                |_| Type::Address,
            ),
            map(parse_integer, Type::Integer),
        ))(code)
    }

    impl Display for Integer {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}{}", if self.signed { "i" } else { "u" }, self.width)
        }
    }
}
