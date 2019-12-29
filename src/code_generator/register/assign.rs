use std::collections::BTreeSet;

use nom::lib::std::collections::{BTreeMap, VecDeque};

use crate::code_generator::register::physical_register::REGISTERS;
use crate::code_generator::register::Register;

// todo: 去掉BTreeSet, 所有均直接返回Vec，然后拼接即可
pub fn assign(used_regs: Vec<BTreeSet<&Register>>) -> BTreeMap<&Register, Register> {
    let mut reg_requirements = VecDeque::new();
    for regs in used_regs {
        for reg in regs {
            reg_requirements.push_back(reg);
        }
    }
    let mut result = BTreeMap::new();
    let mut current_assigned = BTreeSet::new();
    let mut next_address = 0u32;
    while !reg_requirements.is_empty() {
        let reg_requirement = reg_requirements.pop_front().unwrap();
        if !result.contains_key(reg_requirement) {
            let assign_to = if current_assigned.iter()
                .filter(|it| {
                    if let Register::PhysicalRegister(r) = it {
                        true
                    } else {
                        false
                    }
                }).count() == REGISTERS.len() {
                next_address += 4;
                Register::Memory(next_address)
            } else {
                REGISTERS.iter()
                    .find(|&it| !current_assigned.contains(it))
                    .unwrap().clone()
            };
            current_assigned.insert(assign_to.clone());
            result.insert(reg_requirement, assign_to.clone());
        }
        let mut to_remove = current_assigned.clone();
        reg_requirements.iter()
            .for_each(|&it| {
                if let Some(r) = result.get(it) {
                    to_remove.remove(r);
                }
            });
        for r in to_remove {
            current_assigned.remove(&r);
        }
    }
    result
}

#[test]
fn test_assign() {
    let target =
        vec![{
                 let mut t = BTreeSet::new();
                 t.insert(&Register::AbstractRegister(0));
                 t
             }, {
                 let mut t = BTreeSet::new();
                 t.insert(&Register::AbstractRegister(1));
                 t
             }, {
                 let mut t = BTreeSet::new();
                 t.insert(&Register::AbstractRegister(1));
                 t.insert(&Register::AbstractRegister(0));
                 t
             }, {
                 let mut t = BTreeSet::new();
                 t.insert(&Register::AbstractRegister(2));
                 t
             }, {
                 let mut t = BTreeSet::new();
                 t.insert(&Register::AbstractRegister(3));
                 t
             }, {
                 let mut t = BTreeSet::new();
                 t.insert(&Register::AbstractRegister(3));
                 t.insert(&Register::AbstractRegister(2));
                 t
             }];
    let result = assign(target);
    assert_eq!(result.get(&Register::AbstractRegister(0)).unwrap(), &Register::PhysicalRegister("t3"));
    assert_eq!(result.get(&Register::AbstractRegister(1)).unwrap(), &Register::PhysicalRegister("t4"));
    assert_eq!(result.get(&Register::AbstractRegister(2)).unwrap(), &Register::PhysicalRegister("t3"));
    assert_eq!(result.get(&Register::AbstractRegister(3)).unwrap(), &Register::PhysicalRegister("t4"));
}
