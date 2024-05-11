use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::{
    cpu::addressing_mode::AddressingMode,
    cpu::CPU,
};

pub type InstFun = fn(&Inst, &mut CPU);
pub struct Inst {
    pub name: String,
    pub param: Option<u16>,
    pub mode: AddressingMode,
    pub f: InstFun,
}

impl Inst {

    pub fn run(&self, cpu: &mut CPU) {
        (self.f)(self, cpu)
    }

    pub fn len(&self) -> u16 {
        match self.mode {
            AddressingMode::Implied |
            AddressingMode::Accumulator => 1,
            AddressingMode::Immediate
            | AddressingMode::Relative
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => 2,
            AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 3,
        }
    }
}

impl std::fmt::Debug for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mode {
            AddressingMode::Implied => write!(f, "{}", self.name),
            AddressingMode::Accumulator => write!(f, "{} A", self.name),
            AddressingMode::Relative|
            AddressingMode::Immediate => write!(f, "{} #${:02x}", self.name, self.param.unwrap()),
            AddressingMode::ZeroPage => write!(f, "{} ${:02x}, X", self.name, self.param.unwrap()),
            AddressingMode::ZeroPageX => {
                write!(f, "{} ${:#02x}, Y", self.name, self.param.unwrap())
            }
            AddressingMode::Absolute => write!(f, "{} ${:04x}", self.name, self.param.unwrap()),
            AddressingMode::AbsoluteX => {
                write!(f, "{} ${:#04x}, X", self.name, self.param.unwrap())
            }
            AddressingMode::AbsoluteY => {
                write!(f, "{} ${:#04x}, Y", self.name, self.param.unwrap())
            }
            AddressingMode::IndexedIndirect => {
                write!(f, "{} (${:#02x}, X)", self.name, self.param.unwrap())
            }
            AddressingMode::IndirectIndexed => {
                write!(f, "{} (${:#02x}), Y", self.name, self.param.unwrap())
            }
        }
    }
}

pub struct InstFactory {
    pub name: String,
    pub mode: AddressingMode,
    pub f: InstFun,
}

impl InstFactory {
    pub fn make(&self, bytes: &[u8]) -> Inst {
        Inst {
            name: self.name.clone(),
            param: self.mode.read_param(bytes),
            mode: self.mode,
            f: self.f,
        }
    }
}

struct InstructionInfo {
    name: String,
    f: InstFun,
    opcode_to_addressing_mode: &'static[(u8, AddressingMode)],
}

impl InstructionInfo {
    pub fn new(name: String, f: InstFun, opcode_to_addressing_mode: &'static[(u8, AddressingMode)]) -> Self {
        InstructionInfo {
            name,
            f,
            opcode_to_addressing_mode,
        }
    }
}

macro_rules! instruction_info {
    ($module:ident) => {
        InstructionInfo::new(
            stringify!($module).to_uppercase(),
            crate::instructions::$module::RUN,
            crate::instructions::$module::OPCODE_MAP
        )
    };
}

#[macro_export]
macro_rules! define_flag_inst {
    ($opcode: expr, $flag: expr, $value: expr) => {
        use crate::cpu::addressing_mode::AddressingMode;
        use super::InstFun;
        use crate::cpu::test_util::Flag::*;
        use crate::cpu::test_util::Setter;

        pub const RUN : InstFun = |ins, cpu| {
            $flag.set(cpu, $value);
            cpu.pc += ins.len();
        };
        pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[($opcode, AddressingMode::Implied)];

        #[cfg(test)]
        mod tests {
            use crate::cpu::test_util::TestRunner;
            use crate::cpu::test_util::Flag::*;

            #[test]
            fn test_brk() {
                let mut runner = TestRunner::new();
                runner.set($flag, !$value)
                    .test(&[$opcode])
                    .verify($flag, $value);
                runner.set($flag, $value)
                    .test(&[$opcode])
                    .verify($flag, $value);
            }
        }
    };
}

#[macro_export]
macro_rules! define_jump_inst {
    ($opcode: expr, $flag: expr, $value: expr) => {
        use crate::cpu::addressing_mode::{load_operand, AddressingMode};
        use crate::cpu::CPU;
        use super::InstFun;
        use crate::cpu::test_util::Flag::*;
        use crate::cpu::test_util::Retriever;

        pub const RUN : InstFun = |ins, cpu: &mut CPU| {
            let operand : i8 = load_operand(ins.mode, cpu, ins.param.unwrap()) as i8;
            if $flag.get(cpu) == $value {
                cpu.pc = cpu.pc.wrapping_add(operand as u16);
            }
            cpu.pc = cpu.pc.wrapping_add(ins.len());
        };

        pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[
                ($opcode, AddressingMode::Relative),
            ];

        #[cfg(test)]
        mod test {
            use crate::cpu::test_util::TestRunner;
            use crate::cpu::test_util::Register16::*;
            use crate::cpu::test_util::Flag::*;

            #[test]
            fn test_relative() {
                let mut runner = TestRunner::new();
                runner.set($flag, $value);
                runner.set(PC, 0x8000);
                runner.test(&[$opcode, 0x01])
                    .verify(PC, 0x8003);
                runner.set(PC, 0x8000);
                runner.test(&[$opcode, 0x80])
                    .verify(PC, 0x7f82);
                runner.set(PC, 0x8000);
                runner.test(&[$opcode, 0xff])
                    .verify(PC, 0x8001);
                runner.set($flag, !$value);
                runner.test(&[$opcode, 0xff])
                    .verify(PC, 0x8002);
            }
        }
    }
}

pub fn get_opcode(opcode_map: &[(u8, AddressingMode)], mode: AddressingMode) -> u8 {
    for (op, m) in opcode_map {
        if *m == mode {
            return *op;
        }
    }
    assert!(false, "opcode not found");
    return 0;
}

#[macro_export]
macro_rules! defube_cmp_inst {
    ($reg: expr, $opcode_map: expr) => {
        use crate::cpu::{addressing_mode::{load_operand, AddressingMode}, test_util::{Flag, Setter}};
        use super::InstFun;
        use crate::cpu::test_util::Register8::*;
        use crate::cpu::test_util::Retriever;

        pub const RUN : InstFun = |ins, cpu| {
            let operand = load_operand(ins.mode, cpu, ins.param.unwrap());
            let res = $reg.get(&cpu).wrapping_sub(operand);
            Flag::C.set(cpu, $reg.get(&cpu) >= operand);
            Flag::Z.set(cpu, $reg.get(&cpu) == operand);
            Flag::N.set(cpu, res & 0x80 != 0);
            cpu.pc += ins.len();
        };

        pub const OPCODE_MAP: &[(u8, AddressingMode)] = $opcode_map;

        #[cfg(test)]
        mod test {
            use crate::cpu::test_util::TestRunner;
            use crate::cpu::test_util::Register8::*;
            use crate::cpu::test_util::Flag::*;
            use crate::cpu::addressing_mode::AddressingMode;
            use crate::instructions::common::get_opcode;
            use super::OPCODE_MAP;


            #[test]
            fn test_immediate() {
                let mut runner = TestRunner::new();
                let opcode = get_opcode(OPCODE_MAP, AddressingMode::Immediate);
                runner.set($reg, 0x01);
                runner.test(&[opcode, 0x01])
                    .verify(C, true)
                    .verify(Z, true)
                    .verify(N, false);
                runner.set($reg, 0xff);
                runner.test(&[opcode, 0x00])
                    .verify(C, true)
                    .verify(Z, false)
                    .verify(N, true);
                runner.set($reg, 0x03);
                runner.test(&[opcode, 0x02])
                    .verify(C, true)
                    .verify(Z, false)
                    .verify(N, false);
                runner.set($reg, 0x02);
                runner.test(&[opcode, 0x03])
                    .verify(C, false)
                    .verify(Z, false)
                    .verify(N, true);
            }
        }
    }
}
lazy_static! {
pub static ref INST_FACTORIES: HashMap<u8, InstFactory> = {
    let instructions = &[
        instruction_info!(lda),
        instruction_info!(tax),
        instruction_info!(inx),
        instruction_info!(brk),
        instruction_info!(adc),
        instruction_info!(and),
        instruction_info!(asl),
        instruction_info!(bcc),
        instruction_info!(bcs),
        instruction_info!(beq),
        instruction_info!(bit),
        instruction_info!(bmi),
        instruction_info!(bne),
        instruction_info!(bpl),
        instruction_info!(bvc),
        instruction_info!(bvs),
        instruction_info!(clc),
        instruction_info!(sec),
        instruction_info!(cld),
        instruction_info!(sed),
        instruction_info!(cli),
        instruction_info!(sei),
        instruction_info!(clv),
        instruction_info!(cmp),
        instruction_info!(cpx),
        instruction_info!(cpy),
        instruction_info!(dec),
        instruction_info!(dex),
        instruction_info!(dey),
    ];
    let mut inst_factory_by_op_code: HashMap<u8, InstFactory> = HashMap::new();
    for info in instructions.iter() {
        for (op, mode) in info.opcode_to_addressing_mode {
            let res = inst_factory_by_op_code.insert(
                *op,
                InstFactory {
                    mode: *mode,
                    name: info.name.clone(),
                    f: info.f,
                },
            );
            if res.is_some() {
                panic!("duplicate op code: {:#x}", op);
            }
        }
    }
    inst_factory_by_op_code
};
}

pub fn disassemble(bytes: &[u8]) -> Vec<Inst> {
    let mut res = vec![];
    let mut idx = 0;
    while idx < bytes.len() {
        let op = bytes[idx];
        if let Some(factory) = INST_FACTORIES.get(&op) {
            let inst = factory.make(&bytes[(idx + 1)..]);
            idx += inst.len() as usize;
            res.push(inst);
        } else {
            panic!("unknown op code: {:#x}", op);
        }
    }
    res
}
