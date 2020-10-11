use crate::ir;
use crate::ir::store::StoreSource;
use crate::ir::{Register as LogicalRegister, RegisterRef};
use crate::riscv::register::PhysicalRegister;
use std::collections::HashMap;
use std::convert::TryInto;

impl ir::Store {
    pub(crate) fn generate_asm(
        &self,
        register_map: &HashMap<RegisterRef, PhysicalRegister>,
    ) -> String {
        let mut result = Vec::new();
        let operand_real_register = match &self.source {
            StoreSource::NumberLiteral(n) => {
                result.push(format!("li t0, {}", n));
                "t0".to_string()
            }
            StoreSource::Register(register) => {
                let operand_physical_register = register_map.get(&register).unwrap();
                let operand_real_register =
                    operand_physical_register.real_register("t0".to_string());
                let operand_real_register_code =
                    operand_physical_register.must_real_register_read_code("t0".to_string());
                if operand_real_register != "" {
                    result.push(operand_real_register_code);
                }
                operand_real_register
            }
        };
        result.push(match &self.target {
            ir::store::StoreTarget::Local(target) => {
                let target_physical_register = register_map.get(target).unwrap();
                match target_physical_register {
                    PhysicalRegister::RealRegister(reg) => {
                        format!("mv {}, {}", reg.name, operand_real_register)
                    }
                    PhysicalRegister::Memory(offset) => {
                        format!("sw {}, -{}(s0)", operand_real_register, offset)
                    }
                }
            }
            ir::store::StoreTarget::Global(target) => {
                format!("sw {}, .{}", operand_real_register, target)
            }
        });
        result.join("\n").trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::register::{RealRegister, Save};
    use crate::shared::data_type::Integer;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let mut register_map = HashMap::new();
        let logical_register0 = RegisterRef("0".to_string());
        let logical_register1 = RegisterRef("1".to_string());
        let logical_register2 = RegisterRef("2".to_string());
        register_map.insert(
            logical_register0,
            PhysicalRegister::RealRegister(RealRegister {
                id: 1,
                name: "test1".to_string(),
                save: Save::Caller,
            }),
        );
        register_map.insert(
            logical_register1,
            PhysicalRegister::RealRegister(RealRegister {
                id: 2,
                name: "test2".to_string(),
                save: Save::Caller,
            }),
        );
        register_map.insert(logical_register2, PhysicalRegister::Memory(4));
        let load = ir::store::parse("store %0, i32* @a").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "sw test1, .a");

        let load = ir::store::parse("store %0, i32* %1").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "mv test2, test1");

        let load = ir::store::parse("store %0, i32* %2").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "sw test1, -4(s0)");

        let load = ir::store::parse("store %2, i32* @a").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "lw t0, -4(s0)\nsw t0, .a");

        let load = ir::store::parse("store 3, i32* @a").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "li t0, 3\nsw t0, .a");
    }
}
