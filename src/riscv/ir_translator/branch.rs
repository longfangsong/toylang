use crate::ir;
use crate::ir::Register as LogicalRegister;
use crate::riscv::register::PhysicalRegister;
use std::collections::HashMap;

impl ir::Branch {
    pub fn generate_asm(
        &self,
        register_map: &HashMap<&LogicalRegister, PhysicalRegister>,
    ) -> String {
        let operand1_physical_register = register_map.get(&self.operand1).unwrap();
        let operand1_real_register = operand1_physical_register.real_register("t0".to_string());
        let operand1_real_register_code =
            operand1_physical_register.must_real_register_read_code("t0".to_string());
        let operand2_physical_register = register_map.get(&self.operand2).unwrap();
        let operand2_real_register = operand2_physical_register.real_register("t1".to_string());
        let operand2_real_register_code =
            operand2_physical_register.must_real_register_read_code("t1".to_string());
        let mut result = Vec::new();
        if operand1_real_register_code != "" {
            result.push(operand1_real_register_code);
        }
        if operand2_real_register_code != "" {
            result.push(operand2_real_register_code);
        }
        result.push(format!(
            "b{} {}, {}, {}",
            self.branch_type, operand1_real_register, operand2_real_register, self.success_label
        ));
        result.push(format!("j {}", self.failure_label));
        result.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::register::{RealRegister, Save};
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let mut register_map = HashMap::new();
        let logical_register0 = LogicalRegister("0".to_string());
        let logical_register1 = LogicalRegister("1".to_string());
        let logical_register2 = LogicalRegister("2".to_string());
        register_map.insert(
            &logical_register0,
            PhysicalRegister::RealRegister(RealRegister {
                id: 1,
                name: "test1".to_string(),
                save: Save::Caller,
            }),
        );
        register_map.insert(
            &logical_register1,
            PhysicalRegister::RealRegister(RealRegister {
                id: 2,
                name: "test2".to_string(),
                save: Save::Caller,
            }),
        );
        register_map.insert(&logical_register2, PhysicalRegister::Memory(4));
        let branch = ir::branch::parse("blt %0, %1, label1, label2").unwrap().1;
        let asm = branch.generate_asm(&register_map);
        assert_eq!(asm, "blt test1, test2, label1\nj label2");
        let branch = ir::branch::parse("blt %0, %2, label3, label4").unwrap().1;
        let asm = branch.generate_asm(&register_map);
        assert_eq!(asm, "lw t1, -4(s0)\nblt test1, t1, label3\nj label4");
    }
}
