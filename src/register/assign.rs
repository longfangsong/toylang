use nom::lib::std::collections::{HashMap, HashSet, VecDeque};

use crate::register::{PhysicalRegister, REAL_REGISTERS, SSARegister};

pub(crate) fn generate_assign_map(require_sequence: &[SSARegister]) -> HashMap<SSARegister, PhysicalRegister> {
    let mut result = HashMap::new();
    let mut current_using = HashSet::new();
    let mut rest_to_assign: VecDeque<SSARegister> = require_sequence.iter().map(|it| *it).collect();
    let mut next_spill_to_address = 0u32;
    while !rest_to_assign.is_empty() {
        let ssa_register_to_assign = rest_to_assign.pop_front().unwrap();
        if !result.contains_key(&ssa_register_to_assign) {
            let real_register_found = REAL_REGISTERS.iter()
                .find(|&it| !current_using.contains(it))
                .map(|it| it.clone());
            let match_to = real_register_found.unwrap_or_else(|| {
                next_spill_to_address += 4;
                PhysicalRegister::SpilledRegister(next_spill_to_address)
            });
            current_using.insert(match_to);
            result.insert(ssa_register_to_assign, match_to);
        }
        if !rest_to_assign.contains(&ssa_register_to_assign) {
            current_using.remove(result.get(&ssa_register_to_assign).unwrap());
        }
    }
    result
}

#[test]
fn test_generate_assign_map() {
    let require_sequence: Vec<_> = (0..10u64).map(|it| SSARegister(it)).collect();
    let result = generate_assign_map(&require_sequence[..]);
    assert!(result.values().all(|&it| it == PhysicalRegister::RealRegister("t3")));

    let require_sequence: Vec<_> = (0..10u64).cycle().take(20).map(|it| SSARegister(it)).collect();
    let result = generate_assign_map(&require_sequence[..]);
    assert_eq!(result.get(&SSARegister(0)).unwrap(), &PhysicalRegister::RealRegister("t3"));
    assert_eq!(result.get(&SSARegister(1)).unwrap(), &PhysicalRegister::RealRegister("t4"));
}