use crate::code_generator::register::Register;

// "t0" is kept for code generation use
// "t1" is kept for code generation use,
// "t2" is kept for code generation use,
// "s0" is used as fp

pub(crate) const REGISTERS: [Register; 23] = [
    Register::PhysicalRegister("t3"),
    Register::PhysicalRegister("t4"),
    Register::PhysicalRegister("t5"),
    Register::PhysicalRegister("t6"),
    Register::PhysicalRegister("s1"),
    Register::PhysicalRegister("s2"),
    Register::PhysicalRegister("s3"),
    Register::PhysicalRegister("s4"),
    Register::PhysicalRegister("s5"),
    Register::PhysicalRegister("s6"),
    Register::PhysicalRegister("s7"),
    Register::PhysicalRegister("s8"),
    Register::PhysicalRegister("s9"),
    Register::PhysicalRegister("s10"),
    Register::PhysicalRegister("s11"),
    Register::PhysicalRegister("a0"),
    Register::PhysicalRegister("a1"),
    Register::PhysicalRegister("a2"),
    Register::PhysicalRegister("a3"),
    Register::PhysicalRegister("a4"),
    Register::PhysicalRegister("a5"),
    Register::PhysicalRegister("a6"),
    Register::PhysicalRegister("a7"),
];