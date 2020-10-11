use crate::ir;
use crate::ir::load::LoadSource;
use crate::ir::{Register as LogicalRegister, RegisterRef};
use crate::riscv::register::PhysicalRegister;
use std::collections::HashMap;

impl ir::Load {
    pub(crate) fn generate_asm(
        &self,
        register_map: &HashMap<RegisterRef, PhysicalRegister>,
    ) -> String {
        let mut result = Vec::new();
        let target_physical_register = register_map.get(&(&self.to).into()).unwrap();
        let target_real_register = target_physical_register.real_register("t2".to_string());
        let target_real_register_code =
            target_physical_register.must_real_register_write_code("t2".to_string());
        result.push(match &self.from {
            LoadSource::Global(name) => format!("lw {}, .{}", target_real_register, name),
            LoadSource::Local(reg) => {
                let source_real_register = register_map.get(reg).unwrap();
                match source_real_register {
                    PhysicalRegister::RealRegister(reg) => {
                        format!("mv {}, {}", target_real_register, reg.name)
                    }
                    PhysicalRegister::Memory(offset) => {
                        format!("lw {}, -{}(s0)", target_real_register, offset)
                    }
                }
            }
        });
        if target_real_register_code != "" {
            result.push(target_real_register_code);
        }
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
        let load = ir::load::parse("%0 = load i32* @a").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "lw test1, .a");

        let load = ir::load::parse("%0 = load i32* %1").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "mv test1, test2");

        let load = ir::load::parse("%0 = load i32* %2").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "lw test1, -4(s0)");

        let load = ir::load::parse("%2 = load i32* @a").unwrap().1;
        let asm = load.generate_asm(&register_map);
        assert_eq!(asm, "lw t2, .a\nsw t2, -4(s0)");
    }
}
