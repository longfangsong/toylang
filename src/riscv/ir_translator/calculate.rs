use crate::ir;
use crate::ir::calculate::CalculateOperation;
use crate::ir::RegisterRef as LogicalRegisterRef;
use crate::riscv::register::PhysicalRegister;
use std::collections::HashMap;

impl ir::Calculate {
    pub fn generate_asm(
        &self,
        register_map: &HashMap<LogicalRegisterRef, PhysicalRegister>,
    ) -> String {
        let mut result = Vec::new();

        let target_physical_register = register_map.get(&(&(self.to_register)).into()).unwrap();
        let target_real_register = target_physical_register.real_register("t2".to_string());
        let target_real_register_code =
            target_physical_register.must_real_register_write_code("t2".to_string());

        match (&self.operand1, &self.operand2) {
            (
                ir::calculate::Operand::NumberLiteral(num1),
                ir::calculate::Operand::NumberLiteral(num2),
            ) => {
                // both are imm, calculate when compiling
                // todo: transmute, check and so on
                let calculate_result = match self.operation {
                    CalculateOperation::Add => num1 + num2,
                    CalculateOperation::Less => (num1 < num2) as i64,
                    CalculateOperation::Sub => num1 - num2,
                    CalculateOperation::Or => num1 | num2,
                    CalculateOperation::Xor => num1 ^ num2,
                    CalculateOperation::And => num1 & num2,
                    CalculateOperation::SLL => ((*num1 as u64) << (*num2 as u64)) as i64,
                    CalculateOperation::SRL => ((*num1 as u64) >> (*num2 as u64)) as i64,
                    CalculateOperation::SRA => num1 >> num2,
                };
                result.push(format!("li {}, {}", target_real_register, calculate_result))
            }
            (
                ir::calculate::Operand::NumberLiteral(num1),
                ir::calculate::Operand::Register(reg),
            ) => {
                // imm op register
                let operand2_physical_register = register_map.get(reg).unwrap();
                let operand2_real_register =
                    operand2_physical_register.real_register("t1".to_string());
                let operand2_real_register_code =
                    operand2_physical_register.must_real_register_read_code("t1".to_string());
                if operand2_real_register != "" {
                    result.push(operand2_real_register_code);
                }
                let calculate_code = match self.operation {
                    // these four are commutative
                    CalculateOperation::Add => format!(
                        "addi {}, {}, {}",
                        target_real_register, operand2_real_register, num1
                    ),
                    CalculateOperation::Or => format!(
                        "ori {}, {}, {}",
                        target_real_register, operand2_real_register, num1
                    ),
                    CalculateOperation::Xor => format!(
                        "xori {}, {}, {}",
                        target_real_register, operand2_real_register, num1
                    ),
                    CalculateOperation::And => format!(
                        "andi {}, {}, {}",
                        target_real_register, operand2_real_register, num1
                    ),
                    // these five are not
                    CalculateOperation::Less => {
                        // operand2_real_register will use t1, so t0 is free
                        format!(
                            "li t0, {}\nslt {}, t0, {}",
                            num1, target_real_register, operand2_real_register
                        )
                    }
                    CalculateOperation::Sub => format!(
                        "li t0, {}\nsub {}, t0, {}",
                        num1, target_real_register, operand2_real_register
                    ),
                    CalculateOperation::SLL => format!(
                        "li t0, {}\nsll {}, t0, {}",
                        num1, target_real_register, operand2_real_register
                    ),
                    CalculateOperation::SRL => format!(
                        "li t0, {}\nsrl {}, t0, {}",
                        num1, target_real_register, operand2_real_register
                    ),
                    CalculateOperation::SRA => format!(
                        "li t0, {}\nsra {}, t0, {}",
                        num1, target_real_register, operand2_real_register
                    ),
                };
                result.push(calculate_code);
            }
            (
                ir::calculate::Operand::Register(reg),
                ir::calculate::Operand::NumberLiteral(num2),
            ) => {
                // reg op imm
                let operand1_physical_register = register_map.get(reg).unwrap();
                let operand1_real_register =
                    operand1_physical_register.real_register("t0".to_string());
                let operand1_real_register_code =
                    operand1_physical_register.must_real_register_read_code("t0".to_string());
                if operand1_real_register != "" {
                    result.push(operand1_real_register_code);
                }
                let calculate_code = match self.operation {
                    CalculateOperation::Add => format!(
                        "addi {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::Less => format!(
                        "slti {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::Sub => format!(
                        "addi {}, {}, -{}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::Or => format!(
                        "ori {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::Xor => format!(
                        "xori {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::And => format!(
                        "andi {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::SLL => format!(
                        "slli {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::SRL => format!(
                        "srli {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                    CalculateOperation::SRA => format!(
                        "srai {}, {}, {}",
                        target_real_register, operand1_real_register, num2
                    ),
                };
                result.push(calculate_code);
            }
            (ir::calculate::Operand::Register(reg1), ir::calculate::Operand::Register(reg2)) => {
                let operand1_physical_register = register_map.get(reg1).unwrap();
                let operand1_real_register =
                    operand1_physical_register.real_register("t0".to_string());
                let operand1_real_register_code =
                    operand1_physical_register.must_real_register_read_code("t0".to_string());
                let operand2_physical_register = register_map.get(reg2).unwrap();
                let operand2_real_register =
                    operand2_physical_register.real_register("t1".to_string());
                let operand2_real_register_code =
                    operand2_physical_register.must_real_register_read_code("t1".to_string());
                if operand1_real_register != "" {
                    result.push(operand1_real_register_code);
                }
                if operand2_real_register != "" {
                    result.push(operand2_real_register_code);
                }
                let calculate_code = match self.operation {
                    CalculateOperation::Add => "add",
                    CalculateOperation::Less => "slt",
                    CalculateOperation::Sub => "sub",
                    CalculateOperation::Or => "or",
                    CalculateOperation::Xor => "xor",
                    CalculateOperation::And => "and",
                    CalculateOperation::SLL => "sll",
                    CalculateOperation::SRL => "srl",
                    CalculateOperation::SRA => "sra",
                }
                .to_string()
                    + &format!(
                        " {}, {}, {}",
                        target_real_register, operand1_real_register, operand2_real_register
                    );
                result.push(calculate_code);
            }
        };
        result.push(target_real_register_code);
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
        let logical_register0 = LogicalRegisterRef("0".to_string());
        let logical_register1 = LogicalRegisterRef("1".to_string());
        let logical_register2 = LogicalRegisterRef("2".to_string());
        let logical_register3 = LogicalRegisterRef("3".to_string());
        let logical_register4 = LogicalRegisterRef("4".to_string());
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
        register_map.insert(logical_register3, PhysicalRegister::Memory(8));
        register_map.insert(logical_register4, PhysicalRegister::Memory(12));
        let calculate = ir::calculate::parse("%0 = add i32 %1, %2").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(asm, "lw t1, -4(s0)\nadd test1, test2, t1");

        let calculate = ir::calculate::parse("%2 = sub i32 %0, %1").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(asm, "sub t2, test1, test2\nsw t2, -4(s0)");

        let calculate = ir::calculate::parse("%2 = xor i32 %3, %4").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(
            asm,
            "lw t0, -8(s0)\nlw t1, -12(s0)\nxor t2, t0, t1\nsw t2, -4(s0)"
        );

        let calculate = ir::calculate::parse("%0 = add i32 %1, 99").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(asm, "addi test1, test2, 99");

        let calculate = ir::calculate::parse("%0 = add i32 88, %1").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(asm, "addi test1, test2, 88");

        let calculate = ir::calculate::parse("%0 = sub i32 88, %1").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(asm, "li t0, 88\nsub test1, t0, test2");

        let calculate = ir::calculate::parse("%0 = sub i32 %1, 88").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(asm, "addi test1, test2, -88");

        let calculate = ir::calculate::parse("%0 = add i32 11, 22").unwrap().1;
        let asm = calculate.generate_asm(&register_map);
        assert_eq!(asm, "li test1, 33");
    }
}
