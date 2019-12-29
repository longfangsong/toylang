use nom::lib::std::cmp::Ordering;

use crate::code_generator::register::physical_register::REGISTERS;

pub(crate) mod abstract_register;
pub(crate) mod physical_register;
pub(crate) mod assign;

#[derive(Debug, PartialEq, Ord, Eq, Clone)]
pub enum Register {
    AbstractRegister(u64),
    PhysicalRegister(&'static str),
    Memory(u32),
}

impl PartialOrd for Register {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (&Register::AbstractRegister(i), &Register::AbstractRegister(j)) => i.cmp(&j),
            (_, &Register::AbstractRegister(_)) => Ordering::Greater,
            (&Register::AbstractRegister(_), _) => Ordering::Less,
            (&Register::PhysicalRegister(_), &Register::PhysicalRegister(_)) => {
                REGISTERS.iter().enumerate()
                    .find(|it| it.1 == self)
                    .map(|it| it.0)
                    .unwrap().cmp(
                    &REGISTERS.iter().enumerate()
                        .find(|it| it.1 == other)
                        .map(|it| it.0).unwrap())
            }
            (&Register::Memory(_), &Register::PhysicalRegister(_)) => Ordering::Greater,
            (&Register::PhysicalRegister(_), &Register::Memory(_)) => Ordering::Less,
            (&Register::Memory(i), &Register::Memory(j)) => i.cmp(&j)
        })
    }
}

pub(crate) fn make_sure_in_real_reg<'a>(register: &Register, real_reg: &'a str) -> (String, &'a str) {
    match register {
        Register::Memory(offset) => (format!("lw {}, -{}(fp)\n", real_reg, offset), real_reg),
        Register::PhysicalRegister(name) => ("".to_string(), name),
        _ => panic!("Should assign register first!")
    }
}

pub(crate) fn put_real_reg_back(register: &Register, real_reg: &str) -> String {
    match register {
        Register::Memory(offset) => format!("sw {}, -{}(fp)\n", real_reg, offset),
        Register::PhysicalRegister(_) => "".to_string(),
        _ => panic!("Should assign register first!")
    }
}