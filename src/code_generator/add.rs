use std::collections::BTreeSet;

use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::BTreeMap;
use nom::sequence::tuple;

use crate::code_generator::{CodeGenerator, register};
use crate::code_generator::register::{make_sure_in_real_reg, put_real_reg_back, Register};

#[derive(Debug, Clone)]
pub(crate) struct Add {
    op1: register::Register,
    op2: register::Register,
    result: register::Register,
}

pub(crate) fn parse(ir: &str) -> IResult<&str, Add> {
    map(tuple((
        register::abstract_register::parse,
        space0, tag("="), space0,
        tag("add"),
        space0, register::abstract_register::parse, space0,
        tag(","),
        space0, register::abstract_register::parse, space0,
        tag(";"))), |(result,
                         _, _, _,
                         _,
                         _, op1, _,
                         _,
                         _, op2, _,
                         _)| {
        Add {
            op1: register::Register::AbstractRegister(op1),
            op2: register::Register::AbstractRegister(op2),
            result: register::Register::AbstractRegister(result),
        }
    })(ir)
}

impl<'a> CodeGenerator<'a> for Add {
    fn generate_asm(&self) -> String {
        let (real_op1_code, real_op1_reg) = make_sure_in_real_reg(&self.op1, "t0");
        let (real_op2_code, real_op2_reg) = make_sure_in_real_reg(&self.op2, "t1");
        let (_, result_reg) = make_sure_in_real_reg(&self.result, "t2");
        real_op1_code + &real_op2_code[..] +
            &format!("add {}, {}, {}\n", result_reg, real_op1_reg, real_op2_reg)[..] +
            &put_real_reg_back(&self.result, result_reg)
    }

    fn using_regs(&self) -> BTreeSet<&Register> {
        let mut result = BTreeSet::new();
        result.insert(&self.result);
        result.insert(&self.op1);
        result.insert(&self.op2);
        result
    }

    fn assign_regs(&self, dict: &BTreeMap<&Register, Register>) -> Box<dyn CodeGenerator<'a>> {
        Box::new(Add {
            op1: dict.get(&self.op1).unwrap().clone(),
            op2: dict.get(&self.op2).unwrap().clone(),
            result: dict.get(&self.result).unwrap().clone(),
        })
    }
}

#[test]
fn test_parse() {
    let result = parse("%6 = add %4, %5;");
    assert_eq!(result.as_ref().unwrap().1.op1, register::Register::AbstractRegister(4));
    assert_eq!(result.as_ref().unwrap().1.op2, register::Register::AbstractRegister(5));
    assert_eq!(result.as_ref().unwrap().1.result, register::Register::AbstractRegister(6));
}

#[test]
fn test_generate_asm() {
    let mut result = parse("%6 = add %4, %5;").unwrap().1;
    result.op1 = Register::PhysicalRegister("t3");
    result.op2 = Register::PhysicalRegister("t4");
    result.result = Register::PhysicalRegister("t5");
    assert_eq!("add t5, t3, t4\n", result.generate_asm());

    let mut result = parse("%6 = add %4, %5;").unwrap().1;
    result.op1 = Register::PhysicalRegister("t3");
    result.op2 = Register::PhysicalRegister("t3");
    result.result = Register::PhysicalRegister("t3");
    assert_eq!("add t3, t3, t3\n", result.generate_asm());

    result.op1 = Register::PhysicalRegister("t3");
    result.op2 = Register::PhysicalRegister("t4");
    result.result = Register::Memory(4);
    assert_eq!("add t2, t3, t4\nsw t2, -4(fp)\n", result.generate_asm());

    result.op1 = Register::PhysicalRegister("t3");
    result.op2 = Register::Memory(4);
    result.result = Register::PhysicalRegister("t5");
    assert_eq!("lw t1, -4(fp)\nadd t5, t3, t1\n", result.generate_asm());

    result.op1 = Register::PhysicalRegister("t3");
    result.op2 = Register::Memory(4);
    result.result = Register::Memory(8);
    assert_eq!("lw t1, -4(fp)\nadd t2, t3, t1\nsw t2, -8(fp)\n", result.generate_asm());

    result.op1 = Register::Memory(4);
    result.op2 = Register::PhysicalRegister("t4");
    result.result = Register::PhysicalRegister("t5");
    assert_eq!("lw t0, -4(fp)\nadd t5, t0, t4\n", result.generate_asm());

    result.op1 = Register::Memory(4);
    result.op2 = Register::PhysicalRegister("t4");
    result.result = Register::Memory(8);
    assert_eq!("lw t0, -4(fp)\nadd t2, t0, t4\nsw t2, -8(fp)\n", result.generate_asm());

    result.op1 = Register::Memory(4);
    result.op2 = Register::Memory(8);
    result.result = Register::Memory(12);
    assert_eq!("lw t0, -4(fp)\nlw t1, -8(fp)\nadd t2, t0, t1\nsw t2, -12(fp)\n", result.generate_asm());
}