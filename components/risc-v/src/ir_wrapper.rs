use crate::register::AllocatedRegister;
use ir::calculate::{CalculateOperation, Operand};
use ir::load::LoadSource;
use std::collections::HashMap;

pub trait RiscVIR {
    fn generate_asm(&self, register_map: &HashMap<String, AllocatedRegister>) -> String;
}

impl RiscVIR for ir::jump::Jump {
    fn generate_asm(&self, register_map: &HashMap<String, AllocatedRegister>) -> String {
        format!("j {}", self.label)
    }
}

impl RiscVIR for ir::branch::Branch {
    fn generate_asm(&self, register_map: &HashMap<String, AllocatedRegister>) -> String {
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

impl RiscVIR for ir::calculate::Calculate {
    fn generate_asm(&self, register_map: &HashMap<String, AllocatedRegister>) -> String {
        let mut result = Vec::new();

        let target_physical_register = register_map.get(&self.to_register).unwrap();
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
                        format!("li t0, {}\nslt t0, {}", num1, operand2_real_register)
                    }
                    CalculateOperation::Sub => {
                        format!("li t0, {}\nsub t0, {}", num1, operand2_real_register)
                    }
                    CalculateOperation::SLL => {
                        format!("li t0, {}\nsll t0, {}", num1, operand2_real_register)
                    }
                    CalculateOperation::SRL => {
                        format!("li t0, {}\nsrl t0, {}", num1, operand2_real_register)
                    }
                    CalculateOperation::SRA => {
                        format!("li t0, {}\nsra t0, {}", num1, operand2_real_register)
                    }
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
                    CalculateOperation::Add => format!("addi {}, {}", operand1_real_register, num2),
                    CalculateOperation::Less => {
                        format!("slti {}, {}", operand1_real_register, num2)
                    }
                    CalculateOperation::Sub => {
                        format!("addi {}, -{}", operand1_real_register, num2)
                    }
                    CalculateOperation::Or => format!("ori {}, {}", operand1_real_register, num2),
                    CalculateOperation::Xor => format!("xori {}, {}", operand1_real_register, num2),
                    CalculateOperation::And => format!("andi {}, {}", operand1_real_register, num2),
                    CalculateOperation::SLL => format!("slli {}, {}", operand1_real_register, num2),
                    CalculateOperation::SRL => format!("srli {}, {}", operand1_real_register, num2),
                    CalculateOperation::SRA => format!("srai {}, {}", operand1_real_register, num2),
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
                    + &format!(" {}, {}", operand1_real_register, operand2_real_register);
                result.push(calculate_code);
            }
        };
        result.push(target_real_register_code);
        result.join("\n")
    }
}

impl RiscVIR for ir::store::Store {
    fn generate_asm(&self, register_map: &HashMap<String, AllocatedRegister>) -> String {
        let mut result = Vec::new();
        let operand_physical_register = register_map.get(&self.value_register).unwrap();
        let operand_real_register = operand_physical_register.real_register("t0".to_string());
        let operand_real_register_code =
            operand_physical_register.must_real_register_read_code("t0".to_string());
        if operand_real_register != "" {
            result.push(operand_real_register_code);
        }

        result.push(
            if let ir::store::StoreTarget::Local(target) = &self.target {
                let target_physical_register = register_map.get(target).unwrap();
                match target_physical_register {
                    AllocatedRegister::RealRegister(reg) => {
                        format!("mv {}, {}", reg.name, operand_real_register)
                    }
                    AllocatedRegister::Memory(offset) => {
                        format!("sw {}, -{}(s0)", operand_real_register, offset)
                    }
                }
            } else if let ir::store::StoreTarget::Global(target) = &self.target {
                format!("sw {}, .{}", operand_real_register, target)
            } else {
                "".to_string()
            },
        );
        result.join("\n")
    }
}

impl RiscVIR for ir::load::Load {
    fn generate_asm(&self, register_map: &HashMap<String, AllocatedRegister>) -> String {
        let mut result = Vec::new();
        let target_physical_register = register_map.get(&self.to_register).unwrap();
        let target_real_register = target_physical_register.real_register("t2".to_string());
        let target_real_register_code =
            target_physical_register.must_real_register_write_code("t2".to_string());
        result.push(match &self.from {
            LoadSource::Global(name) => format!("lw {}, .{}", target_real_register, name),
            LoadSource::Local(reg) => {
                let source_real_register = register_map.get(reg).unwrap();
                match source_real_register {
                    AllocatedRegister::RealRegister(reg) => {
                        format!("mv {}, {}", target_real_register, reg.name)
                    }
                    AllocatedRegister::Memory(offset) => {
                        format!("lw {}, -{}(s0)", target_real_register, offset)
                    }
                }
            }
        });
        if target_real_register_code != "" {
            result.push(target_real_register_code);
        }
        result.join("\n")
    }
}
