#[macro_use]
extern crate sum_type;

use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

mod alloca;
mod branch;
mod calculate;
mod global;
mod jump;
mod label;
mod load;
mod store;

pub trait RegisterCreator {
    fn created(&self) -> &str;
}

pub trait RegisterUser {
    fn used(&self) -> Vec<&str>;
}

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum IR {
        Alloca(alloca::Alloca),
        Store(store::Store),
        Load(load::Load),
        Calculate(calculate::Calculate),
        Global(global::Global),
        Branch(branch::Branch),
        Jump(jump::Jump),
        Label(label::Label),
    }
}

impl IR {
    pub fn created(&self) -> Option<&str> {
        match self {
            IR::Alloca(u) => Some(u.created()),
            IR::Load(u) => Some(u.created()),
            IR::Calculate(u) => Some(u.created()),
            _ => None,
        }
    }
    pub fn used(&self) -> Vec<&str> {
        match self {
            IR::Store(u) => u.used(),
            IR::Load(u) => u.used(),
            IR::Calculate(u) => u.used(),
            IR::Branch(u) => u.used(),
            _ => vec![],
        }
    }
}

pub fn ir(code: &str) -> IResult<&str, IR> {
    alt((
        map(alloca::alloca, IR::Alloca),
        map(store::store, IR::Store),
        map(load::load, IR::Load),
        map(calculate::calculate, IR::Calculate),
        map(global::global, IR::Global),
        map(branch::branch, IR::Branch),
        map(jump::jump, IR::Jump),
        map(label::label, IR::Label),
    ))(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::character::complete::{line_ending, multispace0};
    use nom::combinator::opt;
    use nom::multi::many0;
    use nom::sequence::tuple;
    use std::convert::TryInto;

    #[test]
    fn it_works() {
        let code = include_str!("../test.ir");
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
                data_type: "i32".to_string(),
                initial_value: 1,
            }
        );

        let label_item: label::Label = ir[2].clone().try_into().unwrap();
        assert_eq!(label_item, label::Label("back".to_string()));

        assert_eq!(ir[3].created(), Some("1"));
        let alloca_item: alloca::Alloca = ir[3].clone().try_into().unwrap();
        assert_eq!(
            alloca_item,
            alloca::Alloca {
                to_register: "1".to_string(),
                data_type: "i32".to_string(),
            }
        );

        assert_eq!(ir[4].created(), Some("2"));
        let load_item: load::Load = ir[4].clone().try_into().unwrap();
        assert_eq!(
            load_item,
            load::Load {
                from: load::LoadSource::Global("a".to_string()),
                to_register: "2".to_string(),
                data_type: "i32".to_string(),
            }
        );

        assert_eq!(ir[5].used().len(), 2);
        assert!(ir[5].used().iter().any(|&it| it == "1"));
        assert!(ir[5].used().iter().any(|&it| it == "2"));
        let store_item: store::Store = ir[5].clone().try_into().unwrap();
        assert_eq!(
            store_item,
            store::Store {
                data_type: "i32".to_string(),
                value_register: "2".to_string(),
                target: store::StoreTarget::Local("1".to_string()),
            }
        );

        assert_eq!(ir[7].used().len(), 1);
        assert_eq!(ir[7].used()[0], "3");
        let store_item: store::Store = ir[7].clone().try_into().unwrap();
        assert_eq!(
            store_item,
            store::Store {
                data_type: "i32".to_string(),
                value_register: "3".to_string(),
                target: store::StoreTarget::Global("a".to_string()),
            }
        );

        assert_eq!(ir[12].used().len(), 2);
        assert!(ir[12].used().iter().any(|&it| it == "5"));
        assert!(ir[12].used().iter().any(|&it| it == "6"));
        let branch_item: branch::Branch = ir[12].clone().try_into().unwrap();
        assert_eq!(
            branch_item,
            branch::Branch {
                branch_type: branch::BranchType::LT,
                operand1: "5".to_string(),
                operand2: "6".to_string(),
                success_label: "back".to_string(),
                failure_label: "next".to_string(),
            }
        );

        assert_eq!(ir[14].used().len(), 2);
        assert!(ir[14].used().iter().any(|&it| it == "5"));
        assert!(ir[14].used().iter().any(|&it| it == "6"));
        assert_eq!(ir[14].created(), Some("7"));
        let calculate_item: calculate::Calculate = ir[14].clone().try_into().unwrap();
        assert_eq!(
            calculate_item,
            calculate::Calculate {
                operation: calculate::CalculateOperation::Add,
                operand1: calculate::Operand::Register("5".to_string()),
                operand2: calculate::Operand::Register("6".to_string()),
                to_register: "7".to_string(),
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
