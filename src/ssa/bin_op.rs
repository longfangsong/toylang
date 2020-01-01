use std::collections::HashMap;
use std::fmt::Display;

use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::Err;
use nom::error::ErrorKind;
use nom::IResult;
use nom::lib::std::fmt::{Error, Formatter};
use nom::sequence::tuple;

use crate::code::expression::bin_op::OP_NAME;
use crate::register::{PhysicalRegister, SSARegister};
use crate::ssa::SSAStatement;

pub(crate) struct BinOp {
    pub(crate) result: SSARegister,
    pub(crate) lhs: SSARegister,
    pub(crate) op: &'static str,
    pub(crate) rhs: SSARegister,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{} = add {}, {};", self.result, self.lhs, self.rhs)
    }
}

impl SSAStatement for BinOp {
    fn require_registers(&self) -> Vec<SSARegister> {
        vec![self.lhs, self.rhs, self.result]
    }

    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister>) -> String {
        let result = register_map.get(&self.result).unwrap();
        let lhs = register_map.get(&self.lhs).unwrap();
        let rhs = register_map.get(&self.rhs).unwrap();

        let result_reg = result.real_reg_or("t0");
        let lhs_reg = lhs.real_reg_or("t1");
        let rhs_reg = rhs.real_reg_or("t2");
        lhs.load_reg_code("t1") +
            &rhs.load_reg_code("t2") +
            &format!("{} {}, {}, {}\n",
                     self.op, result_reg, lhs_reg, rhs_reg) +
            &result.store_reg_code("t0")
    }
}

fn parse_op(code: &str) -> IResult<&str, &'static str> {
    for &op in OP_NAME.values() {
        let this_match: IResult<&str, &str> = tag(op)(code);
        if this_match.is_ok() {
            return Ok((this_match.unwrap().0, op));
        }
    }
    Err(Err::Error(("No op found", ErrorKind::Alt)))
}

#[test]
fn test_parse_op() {
    let result = parse_op("add %1, %2;").unwrap();
    assert_eq!(result.1, "add");
    assert_eq!(result.0, " %1, %2;");
    let result = parse_op("asdf");
    assert!(result.is_err());
    let result = parse_op("1234");
    assert!(result.is_err());
    let result = parse_op("+");
    assert!(result.is_err());
}

impl BinOp {
    pub(crate) fn parse(ir: &str) -> IResult<&str, Self> {
        map(tuple((SSARegister::parse,
                   space0, tag("="), space0,
                   parse_op, space0,
                   SSARegister::parse,
                   space0, tag(","), space0,
                   SSARegister::parse,
                   space0, tag(";"))),
            |(to, _, _, _, op, _, op1, _, _, _, op2, _, _)| {
                BinOp {
                    result: to,
                    lhs: op1,
                    op,
                    rhs: op2,
                }
            })(ir)
    }
}

#[test]
fn test_parse() {
    let result = BinOp::parse("%0 = add %1, %2;").unwrap();
    assert_eq!(result.1.result, SSARegister(0));
    assert_eq!(result.1.lhs, SSARegister(1));
    assert_eq!(result.1.rhs, SSARegister(2));
    assert_eq!(result.1.op, "add");

    let result = BinOp::parse("%0 = sth %1, %2;");
    assert!(result.is_err());
}

#[test]
fn test_require_registers() {
    use crate::tools::id_generator::reset_id;

    reset_id();
    let result = BinOp::parse("%0 = add %1, %2;").unwrap().1;
    assert_eq!(result.require_registers().len(), 3);
    assert_eq!(result.require_registers()[0], SSARegister(1));
    assert_eq!(result.require_registers()[1], SSARegister(2));
    assert_eq!(result.require_registers()[2], SSARegister(0));
}

#[test]
fn test_generate_asm() {
    let result = BinOp::parse("%0 = add %1, %2;").unwrap().1;
    let mut register_map = HashMap::new();
    register_map.insert(SSARegister(0), PhysicalRegister::RealRegister("t3"));
    register_map.insert(SSARegister(1), PhysicalRegister::RealRegister("t4"));
    register_map.insert(SSARegister(2), PhysicalRegister::RealRegister("t5"));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "add t3, t4, t5\n");

    register_map.insert(SSARegister(0), PhysicalRegister::SpilledRegister(4));
    register_map.insert(SSARegister(1), PhysicalRegister::RealRegister("t4"));
    register_map.insert(SSARegister(2), PhysicalRegister::SpilledRegister(8));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "lw t2, -8(fp)\nadd t0, t4, t2\nsw t0, -4(fp)\n");
}