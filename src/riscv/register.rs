use crate::ir::IR;
use crate::ir::{Register as LogicRegister, RegisterRef};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Range;
use std::{fmt, mem};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Save {
    Caller,
    Callee,
}

impl Serialize for Save {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            Save::Caller => serializer.serialize_str("caller"),
            Save::Callee => serializer.serialize_str("callee"),
        }
    }
}

struct SaveVisitor;

impl<'de> Visitor<'de> for SaveVisitor {
    type Value = Save;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting \"caller\" or \"callee\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "caller" => Ok(Save::Caller),
            "callee" => Ok(Save::Callee),
            _ => Err(E::custom("expecting \"caller\" or \"callee\"".to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for Save {
    fn deserialize<D>(deserializer: D) -> Result<Save, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SaveVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct RealRegister {
    pub id: u8,
    pub name: String,
    pub save: Save,
}

lazy_static! {
    pub static ref REGISTERS: VecDeque<RealRegister> =
        serde_yaml::from_str(include_str!("./registers.yaml")).unwrap();
}

#[derive(Debug)]
pub enum PhysicalRegister {
    RealRegister(RealRegister),
    Memory(usize),
}

pub struct RegisterAllocationTable(HashMap<RegisterRef, PhysicalRegister>);

pub fn allocate_registers(irs: &[IR]) -> HashMap<RegisterRef, PhysicalRegister> {
    // find out when each logical register is active
    let mut active_intervals: HashMap<RegisterRef, Range<usize>> = HashMap::new();
    for (index, ir) in irs.iter().enumerate() {
        for used_register in ir.use_registers() {
            if let Some(old_active_range) = active_intervals.get_mut(used_register) {
                old_active_range.end = index + 1;
            } else {
                active_intervals.insert(used_register.clone(), index..index + 1);
            }
        }
    }

    let mut result: HashMap<RegisterRef, PhysicalRegister> = HashMap::new();
    let mut available_real_registers = REGISTERS.clone();
    let mut active_logic_registers: HashSet<RegisterRef> = HashSet::new();
    let mut used_address = 0; // todo: reuse recycled memory
    for (index, ir) in irs.iter().enumerate() {
        // alloca a physical register for new logical register
        if let Some(created_register) = ir.create_register() {
            active_logic_registers.insert(created_register.into());
            let allocated = available_real_registers
                .pop_front()
                .map(PhysicalRegister::RealRegister)
                .unwrap_or_else(|| {
                    used_address += 4;
                    PhysicalRegister::Memory(used_address)
                });
            result.insert(created_register.into(), allocated);
        }
        let old_active_logic_registers = mem::take(&mut active_logic_registers);
        for old_active_logic_register in old_active_logic_registers {
            if active_intervals
                .get(&old_active_logic_register)
                .unwrap()
                .contains(&index)
            {
                active_logic_registers.insert(old_active_logic_register);
            } else {
                // `active_logic_register` now goes out of lifetime
                // recycle it's physical register
                let new_empty_register = result.get(&old_active_logic_register).unwrap();
                if let PhysicalRegister::RealRegister(real_register) = new_empty_register {
                    available_real_registers.push_back(real_register.clone());
                }
            }
        }
    }
    result
}

impl PhysicalRegister {
    pub fn real_register(&self, backup_register: String) -> String {
        if let PhysicalRegister::Memory(_offset) = self {
            backup_register
        } else if let PhysicalRegister::RealRegister(real_register) = self {
            real_register.name.clone()
        } else {
            unreachable!()
        }
    }
    // return code, name
    pub fn must_real_register_read_code(&self, backup_register: String) -> String {
        if let PhysicalRegister::Memory(offset) = self {
            format!("lw {}, -{}(s0)", backup_register, offset)
        } else if let PhysicalRegister::RealRegister(_real_register) = self {
            "".to_string()
        } else {
            unreachable!()
        }
    }

    pub fn must_real_register_write_code(&self, backup_register: String) -> String {
        if let PhysicalRegister::Memory(offset) = self {
            format!("sw {}, -{}(s0)", backup_register, offset)
        } else if let PhysicalRegister::RealRegister(_real_register) = self {
            "".to_string()
        } else {
            unreachable!()
        }
    }
}
