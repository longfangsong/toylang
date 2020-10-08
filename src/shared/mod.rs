pub mod data_type {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::map;
    use nom::sequence::pair;
    use nom::IResult;
    use std::fmt;
    use std::fmt::Display;
    use std::str::FromStr;
    use sum_type::_core::fmt::Formatter;

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Integer {
        pub signed: bool,
        pub width: usize,
    }

    pub fn parse(code: &str) -> IResult<&str, Integer> {
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

    impl Display for Integer {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}{}", if self.signed { "i" } else { "u" }, self.width)
        }
    }
}
