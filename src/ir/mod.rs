use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

pub mod alloca;
pub mod branch;
pub mod calculate;
pub mod global;
pub mod jump;
pub mod label;
pub mod load;
pub mod phi;
pub mod register;
pub mod store;

pub use crate::ir::register::{Register, RegisterRef};
pub use alloca::Alloca;
pub use branch::Branch;
pub use calculate::Calculate;
use fmt::Formatter;
pub use global::Global;
pub use jump::Jump;
pub use label::Label;
pub use load::Load;
pub use phi::Phi;
use std::fmt;
use std::fmt::Display;
pub use store::Store;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum IR {
        Alloca,
        Store,
        Load,
        Calculate,
        Global,
        Branch,
        Jump,
        Label,
        Phi,
    }
}

impl Display for IR {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IR::Alloca(alloca) => write!(f, "{}", alloca),
            IR::Store(alloca) => write!(f, "{}", alloca),
            IR::Load(alloca) => write!(f, "{}", alloca),
            IR::Calculate(alloca) => write!(f, "{}", alloca),
            IR::Global(alloca) => write!(f, "{}", alloca),
            IR::Branch(alloca) => write!(f, "{}", alloca),
            IR::Jump(alloca) => write!(f, "{}", alloca),
            IR::Label(alloca) => write!(f, "{}", alloca),
            IR::Phi(phi) => write!(f, "{}", phi),
        }
    }
}

impl IR {
    pub fn create_register(&self) -> Option<&Register> {
        match self {
            IR::Load(it) => Some(it.create_register()),
            IR::Calculate(it) => Some(it.create_register()),
            _ => None,
        }
    }
    pub fn use_registers(&self) -> Vec<&RegisterRef> {
        match self {
            IR::Store(it) => it.use_registers(),
            IR::Load(it) => it.use_registers(),
            IR::Calculate(it) => it.use_registers(),
            IR::Branch(it) => it.use_registers(),
            IR::Phi(it) => it.use_registers(),
            _ => Vec::new(),
        }
    }
}

pub fn ir(code: &str) -> IResult<&str, IR> {
    alt((
        map(alloca::parse, IR::Alloca),
        map(store::parse, IR::Store),
        map(load::parse, IR::Load),
        map(calculate::parse, IR::Calculate),
        map(global::parse, IR::Global),
        map(branch::parse, IR::Branch),
        map(jump::parse, IR::Jump),
        map(label::parse, IR::Label),
        map(phi::parse, IR::Phi),
    ))(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::data_type::{Integer, Type};
    use nom::character::complete::{line_ending, multispace0};
    use nom::combinator::opt;
    use nom::multi::many0;
    use nom::sequence::tuple;
    use std::convert::TryInto;

    #[test]
    fn it_works() {
        let code = include_str!("./test.ir");
        let parser = |code: &'static str| -> IResult<&'static str, Vec<IR>> {
            many0(map(
                tuple((multispace0, ir, opt(line_ending))),
                |(_, result, _)| result,
            ))(code)
        };
        let (_, ir) = parser(code).unwrap();
        assert_eq!(ir.len(), 16);

        let global_item: global::Global = ir[0].clone().try_into().unwrap();
        assert_eq!(
            global_item,
            global::Global {
                name: "a".to_string(),
                data_type: Integer {
                    signed: false,
                    width: 32,
                },
                initial_value: 1,
            }
        );

        let label_item: label::Label = ir[2].clone().try_into().unwrap();
        assert_eq!(label_item, label::Label("back".to_string()));

        let alloca_item: alloca::Alloca = ir[3].clone().try_into().unwrap();
        assert_eq!(
            alloca_item,
            alloca::Alloca {
                to: register::Register {
                    name: "1".to_string(),
                    data_type: Type::Address,
                },
                alloc_type: Integer {
                    signed: false,
                    width: 32,
                }
                .into(),
            }
        );

        assert_eq!(
            ir[4].create_register(),
            Some(&register::Register {
                name: "2".to_string(),
                data_type: Integer {
                    signed: false,
                    width: 32,
                }
                .into(),
            })
        );
        let load_item: load::Load = ir[4].clone().try_into().unwrap();
        assert_eq!(
            load_item,
            load::Load {
                from: load::LoadSource::Global("a".to_string()),
                to: register::Register {
                    name: "2".to_string(),
                    data_type: Integer {
                        signed: false,
                        width: 32,
                    }
                    .into(),
                },
            }
        );

        assert_eq!(ir[5].use_registers().len(), 2);
        assert!(ir[5].use_registers().iter().any(|&it| it.0 == "1"));
        assert!(ir[5].use_registers().iter().any(|&it| it.0 == "2"));
        let store_item: store::Store = ir[5].clone().try_into().unwrap();
        assert_eq!(
            store_item,
            store::Store {
                source: register::RegisterRef("2".to_string()).into(),
                target: store::StoreTarget::Local(register::RegisterRef("1".to_string())),
            }
        );

        assert_eq!(ir[7].use_registers().len(), 1);
        assert_eq!(
            ir[7].use_registers()[0],
            &register::RegisterRef("3".to_string())
        );
        let store_item: store::Store = ir[7].clone().try_into().unwrap();
        assert_eq!(
            store_item,
            store::Store {
                source: register::RegisterRef("3".to_string()).into(),
                target: store::StoreTarget::Global("a".to_string()),
            }
        );

        assert_eq!(ir[12].use_registers().len(), 2);
        assert!(ir[12].use_registers().iter().any(|&it| it.0 == "5"));
        assert!(ir[12].use_registers().iter().any(|&it| it.0 == "6"));
        let branch_item: branch::Branch = ir[12].clone().try_into().unwrap();
        assert_eq!(
            branch_item,
            branch::Branch {
                branch_type: branch::BranchType::LT,
                operand1: register::RegisterRef("5".to_string()),
                operand2: register::RegisterRef("6".to_string()),
                success_label: "back".to_string(),
                failure_label: "next".to_string(),
            }
        );

        assert_eq!(ir[14].use_registers().len(), 2);
        assert!(ir[14].use_registers().iter().any(|&it| it.0 == "5"));
        assert!(ir[14].use_registers().iter().any(|&it| it.0 == "6"));
        assert_eq!(
            ir[14].create_register(),
            Some(&register::Register {
                name: "7".to_string(),
                data_type: Integer {
                    signed: false,
                    width: 32,
                }
                .into(),
            })
        );
        let calculate_item: calculate::Calculate = ir[14].clone().try_into().unwrap();
        assert_eq!(
            calculate_item,
            calculate::Calculate {
                operation: calculate::CalculateOperation::Add,
                operand1: calculate::Operand::Register(register::RegisterRef("5".to_string())),
                operand2: calculate::Operand::Register(register::RegisterRef("6".to_string())),
                to_register: register::Register {
                    name: "7".to_string(),
                    data_type: Integer {
                        signed: false,
                        width: 32,
                    }
                    .into(),
                },
            }
        );

        let jump_item: jump::Jump = ir[15].clone().try_into().unwrap();
        assert_eq!(
            jump_item,
            jump::Jump {
                label: "back".to_string(),
            }
        );
    }
}
