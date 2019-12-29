use nom::IResult;

pub trait LValue: std::fmt::Debug {
    fn generate_lvalue_ir(&self) -> (String, u64);
}

pub(crate) fn lift<'a, O: 'a + LValue, P>(parser: P) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn 'a + LValue>>
    where P: Fn(&'a str) -> IResult<&'a str, O> {
    move |code: &'a str| -> IResult<&'a str, Box<dyn 'a + LValue>> {
        parser(code).map(|(rest, result)| (rest, Box::new(result) as _))
    }
}
