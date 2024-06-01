use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::{
    cpu::addressing_mode::AddressingMode,
    cpu::CPU,
};

pub type InstFun = fn(&Inst, &mut CPU);

#[derive(Clone, PartialEq)]
pub struct Inst {
    pub opcode: u8,
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
        self.mode.get_inst_size()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.opcode);
        match self.mode {
            AddressingMode::Implied |
            AddressingMode::Accumulator => {}
            AddressingMode::Immediate
            | AddressingMode::Relative
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => {
                bytes.push((self.param.unwrap() & 0xff) as u8);
            }
            AddressingMode::Indirect
            | AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY => {
                bytes.push((self.param.unwrap() & 0xff) as u8);
                bytes.push((self.param.unwrap() >> 8) as u8);
            }
        }
        bytes
    }

    pub fn to_string(&self, pc: Option<u16>) -> String {
        match self.mode {
            AddressingMode::Implied => format!("{}", self.name),
            AddressingMode::Accumulator => format!("{} A", self.name),
            AddressingMode::Relative => {
                let delta = (self.param.unwrap() as i8).wrapping_add(2);
                if let Some(addr) = pc {
                    let target = addr.wrapping_add(delta as u16);
                    format!("{} ${:04X}", self.name, target)
                } else {
                    format!("{} *{:02X}", self.name, delta)
                }
            }
            AddressingMode::Immediate => format!("{} #${:02X}", self.name, self.param.unwrap()),
            AddressingMode::ZeroPage => format!("{} ${:02X}", self.name, self.param.unwrap()),
            AddressingMode::ZeroPageX => {
                format!("{} ${:02X},X", self.name, self.param.unwrap())
            }
            AddressingMode::ZeroPageY => {
                format!("{} ${:02X},Y", self.name, self.param.unwrap())
            }
            AddressingMode::Absolute => format!("{} ${:04X}", self.name, self.param.unwrap()),
            AddressingMode::AbsoluteX => {
                format!("{} ${:04X},X", self.name, self.param.unwrap())
            }
            AddressingMode::AbsoluteY => {
                format!("{} ${:04X},Y", self.name, self.param.unwrap())
            }
            AddressingMode::IndexedIndirect => {
                format!("{} (${:02X},X)", self.name, self.param.unwrap())
            }
            AddressingMode::IndirectIndexed => {
                format!("{} (${:02X}),Y", self.name, self.param.unwrap())
            }
            AddressingMode::Indirect => format!("{} (${:04X})", self.name, self.param.unwrap()),
        }
    }
}

impl std::fmt::Debug for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(None))
    }
}

pub struct InstFactory {
    pub opcode : u8,
    pub name: String,
    pub mode: AddressingMode,
    pub f: InstFun,
}

impl InstFactory {
    // Create an instruction from the given bytes.
    // Bytes does not include the opcode.
    pub fn make(&self, bytes: &[u8]) -> Inst {
        Inst {
            opcode: self.opcode,
            name: self.name.clone(),
            param: self.mode.read_param(bytes),
            mode: self.mode,
            f: self.f,
        }
    }

    pub fn make2(&self, param: Option<u16>) -> Inst {
        Inst {
            opcode: self.opcode,
            name: self.name.clone(),
            param,
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
        use crate::cpu::Flag::*;
        use crate::cpu::Setter;

        pub const RUN : InstFun = |ins, cpu| {
            $flag.set(cpu, $value);
            cpu.pc += ins.len();
        };
        pub const OPCODE_MAP: &[(u8, AddressingMode)] = &[($opcode, AddressingMode::Implied)];

        #[cfg(test)]
        mod tests {
            use crate::cpu::test_util::TestRunner;
            use crate::cpu::Flag::*;

            #[test]
            fn test_brk() {
                let mut runner = TestRunner::new();
                runner.set($flag, !$value)
                    .load_and_test(&[$opcode])
                    .verify($flag, $value);
                runner.set($flag, $value)
                    .load_and_test(&[$opcode])
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
        use crate::cpu::Flag::*;
        use crate::cpu::Retriever;

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
            use crate::cpu::Register16::*;
            use crate::cpu::Flag::*;

            #[test]
            fn test_relative() {
                let mut runner = TestRunner::new();
                runner.set($flag, $value);
                runner.set(PC, 0x8000);
                runner.load_and_test(&[$opcode, 0x01])
                    .verify(PC, 0x8003);
                runner.set(PC, 0x8000);
                runner.load_and_test(&[$opcode, 0x80])
                    .verify(PC, 0x7f82);
                runner.set(PC, 0x8000);
                runner.load_and_test(&[$opcode, 0xff])
                    .verify(PC, 0x8001);
                runner.set($flag, !$value);
                runner.load_and_test(&[$opcode, 0xff])
                    .verify(PC, 0x8002);
            }
        }
    }
}

#[cfg(test)]
pub fn get_opcode(opcode_map: &[(u8, AddressingMode)], mode: AddressingMode) -> Option<u8> {
    for (op, m) in opcode_map {
        if *m == mode {
            return Some(*op);
        }
    }
    None
}

#[macro_export]
macro_rules! defube_cmp_inst {
    ($reg: expr, $opcode_map: expr) => {
        use crate::cpu::{addressing_mode::{load_operand, AddressingMode}, Flag, Setter};
        use super::InstFun;
        use crate::cpu::Register8::*;
        use crate::cpu::Retriever;

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
            use crate::cpu::Register8::*;
            use crate::cpu::Flag::*;
            use crate::cpu::addressing_mode::AddressingMode;
            use crate::instructions::common::get_opcode;
            use super::OPCODE_MAP;


            #[test]
            fn test_immediate() {
                let mut runner = TestRunner::new();
                let opcode = get_opcode(OPCODE_MAP, AddressingMode::Immediate).unwrap();
                runner.set($reg, 0x01);
                runner.load_and_test(&[opcode, 0x01])
                    .verify(C, true)
                    .verify(Z, true)
                    .verify(N, false);
                runner.set($reg, 0xff);
                runner.load_and_test(&[opcode, 0x00])
                    .verify(C, true)
                    .verify(Z, false)
                    .verify(N, true);
                runner.set($reg, 0x03);
                runner.load_and_test(&[opcode, 0x02])
                    .verify(C, true)
                    .verify(Z, false)
                    .verify(N, false);
                runner.set($reg, 0x02);
                runner.load_and_test(&[opcode, 0x03])
                    .verify(C, false)
                    .verify(Z, false)
                    .verify(N, true);
            }
        }
    }
}

#[macro_export]
macro_rules! define_ld_inst {
    ($reg: expr, $opcode_map: expr) => {
        use crate::cpu::addressing_mode::{load_operand, AddressingMode};
        use crate::cpu::CPU;
        use super::InstFun;
        use crate::cpu::Register8::*;
        use crate::cpu::Setter;

        pub const RUN : InstFun = |ins, cpu: &mut CPU| {
            let value = load_operand(ins.mode, cpu, ins.param.unwrap());
            $reg.set(cpu, value);
            cpu.update_z(value);
            cpu.update_n(value);
            cpu.pc += ins.len();
        };

        pub const OPCODE_MAP: &[(u8, AddressingMode)] = $opcode_map;

        #[cfg(test)]
        mod test {
            use crate::cpu::test_util::TestRunner;
            use crate::cpu::Register8::*;
            use crate::cpu::Flag::*;
            use super::OPCODE_MAP;
            use crate::cpu::addressing_mode::AddressingMode;
            use crate::instructions::common::get_opcode;

            #[test]
            fn test_immediate() {
                let mut runner = TestRunner::new();
                let opcode = get_opcode(OPCODE_MAP, AddressingMode::Immediate).unwrap();
                runner.load_and_test(&[opcode, 0x00])
                    .verify($reg, 0)
                    .verify(Z, true)
                    .verify(N, false);
                runner.load_and_test(&[opcode, 0x01])
                    .verify($reg, 1)
                    .verify(Z, false)
                    .verify(N, false);
                runner.load_and_test(&[opcode, 0x91])
                    .verify($reg, 0x91)
                    .verify(Z, false)
                    .verify(N, true);
            }

            #[test]
            fn test_zero_page() {
                let mut runner = TestRunner::new();
                let opcode = get_opcode(OPCODE_MAP, AddressingMode::ZeroPage).unwrap();
                runner.load_and_test(&[opcode, 0x01])
                    .verify($reg, 0)
                    .verify(Z, true)
                    .verify(N, false);
                runner.set_mem(0x01, 10);
                runner.load_and_test(&[opcode, 0x01])
                    .verify($reg, 10)
                    .verify(Z, false)
                    .verify(N, false);
                runner.set_mem(0x01, 0xff);
                runner.load_and_test(&[opcode, 0x01])
                    .verify($reg, 0xff)
                    .verify(Z, false)
                    .verify(N, true);
            }

            #[test]
            fn test_zero_page_x() {
                let mut runner = TestRunner::new();
                if let Some(opcode) = get_opcode(OPCODE_MAP, AddressingMode::ZeroPageX) {
                    runner.set_mem(0x01, 0x00);
                    runner.load_and_test(&[opcode, 0x01])
                        .verify($reg, 0)
                        .verify(Z, true)
                        .verify(N, false);
                    runner.set(X, 2);

                    runner.set_mem(0x03, 10);
                    runner.load_and_test(&[opcode, 0x01])
                        .verify($reg, 10)
                        .verify(Z, false)
                        .verify(N, false);

                    runner.set(X, 0x80);
                    runner.set_mem(0x7f, 0xff);
                    runner.load_and_test(&[opcode, 0xff])
                        .verify($reg, 0xff)
                        .verify(Z, false)
                        .verify(N, true);
                }
            }

            #[test]
            fn test_absolute() {
                let mut runner = TestRunner::new();
                let opcode = get_opcode(OPCODE_MAP, AddressingMode::Absolute).unwrap();
                runner.set_mem(0x1234, 0x11);
                runner.load_and_test(&[opcode, 0x34, 0x12])
                    .verify($reg, 0x11)
                    .verify(Z, false)
                    .verify(N, false);
            }

            #[test]
            fn test_absolute_x() {
                let mut runner = TestRunner::new();
                if let Some(opcode) = get_opcode(OPCODE_MAP, AddressingMode::AbsoluteX) {
                    runner.set_mem(0x1235, 0xf0);
                    runner.set(X, 1);
                    runner.load_and_test(&[opcode, 0x34, 0x12])
                        .verify($reg, 0xf0)
                        .verify(Z, false)
                        .verify(N, true);
                }
            }

            #[test]
            fn test_absolute_y() {
                let mut runner = TestRunner::new();
                if let Some(opcode) = get_opcode(OPCODE_MAP, AddressingMode::AbsoluteY) {
                    runner.set_mem(0x1236, 0x13);
                    runner.set(Y, 2);
                    runner.load_and_test(&[opcode, 0x34, 0x12])
                        .verify($reg, 0x13)
                        .verify(Z, false)
                        .verify(N, false);
                }
            }

            #[test]
            fn test_indexed_indirect() {
                let mut runner = TestRunner::new();
                if let Some(opcode) = get_opcode(OPCODE_MAP, AddressingMode::IndexedIndirect) {
                    runner.set(X, 0x11);
                    runner.set_mem(0x21, 0x12);
                    runner.set_mem(0x22, 0x34);
                    runner.set_mem(0x3412, 0x56);
                    runner.load_and_test(&[opcode, 0x10])
                        .verify($reg, 0x56)
                        .verify(Z, false)
                        .verify(N, false);
                }
            }

            #[test]
            fn test_indirect_indexed() {
                let mut runner = TestRunner::new();
                if let Some(opcode) = get_opcode(OPCODE_MAP, AddressingMode::IndirectIndexed) {
                    runner.set(Y, 0x01);
                    runner.set_mem(0x10, 0x23);
                    runner.set_mem(0x11, 0x20);
                    runner.set_mem(0x2024, 0x45);
                    runner.load_and_test(&[opcode, 0x10])
                        .verify($reg, 0x45)
                        .verify(Z, false)
                        .verify(N, false);
                }
            }

        }
    };
}

#[macro_export]
macro_rules! define_st_inst {
    ($reg: expr, $opcode_map: expr) => {
        use crate::cpu::addressing_mode::{load_operand_addr, AddressingMode};
        use super::InstFun;
        use crate::cpu::Register8::*;
        use crate::cpu::Retriever;

        pub const RUN : InstFun = |ins, cpu| {
            let addr = load_operand_addr(ins.mode, cpu, ins.param.unwrap());
            cpu.set_mem(addr, $reg.get(cpu));
            cpu.pc += ins.len();
        };

        pub const OPCODE_MAP: &[(u8, AddressingMode)] = $opcode_map;

        #[cfg(test)]
        mod tests {
            use crate::cpu::test_util::TestRunner;
            use crate::cpu::Mem;
            use super::*;
            use crate::instructions::common::get_opcode;

            #[test]
            fn test_brk() {
                let mut runner = TestRunner::new();
                let opcode = get_opcode(OPCODE_MAP, AddressingMode::ZeroPage).unwrap();
                runner.set($reg, 0x34)
                    .load_and_test(&[opcode, 0x12])
                    .verify(Mem::new(0x12), 0x34);
            }
        }
    };
}

#[macro_export]
macro_rules! define_t_inst {
    ($src: expr, $dst: expr, $opcode_map: expr) => {
        define_t_inst!($src, $dst, $opcode_map, true);
    };
    ($src: expr, $dst: expr, $opcode_map: expr, $update_status: expr) => {
        use crate::cpu::addressing_mode::AddressingMode;
        use super::InstFun;
        use crate::cpu::Register8::*;
        use crate::cpu::Retriever;
        use crate::cpu::Setter;

        pub const RUN : InstFun = |ins, cpu| {
            let value = $src.get(cpu);
            $dst.set(cpu, value);
            if ($update_status) {
                cpu.update_z(value);
                cpu.update_n(value);
            }
            cpu.pc += ins.len();
        };

        pub const OPCODE_MAP: &[(u8, AddressingMode)] = $opcode_map;

        #[cfg(test)]
        mod test {
            use crate::cpu::test_util::TestRunner;
            use crate::cpu::Flag::*;
            use crate::cpu::Register8::*;
            use super::*;
            use crate::instructions::common::get_opcode;

            #[test]
            fn test() {
                let mut runner = TestRunner::new();
                let opcode = get_opcode(OPCODE_MAP, AddressingMode::Implied).unwrap();
                runner.load_program(&[opcode]);
                runner.set($src, 0x21);
                let test = runner.test();
                test.verify($dst, 0x21);
                if ($update_status) {
                    test.verify(Z, false);
                    test.verify(N, false);
                }

                runner.load_program(&[opcode]);
                runner.set($src, 0);
                let test = runner.test();
                test.verify($dst, 0);
                if ($update_status) {
                    test.verify(Z, true);
                    test.verify(N, false);
                }

                runner.load_program(&[opcode]);
                runner.set($src, 0xf0);
                let test = runner.test();
                test.verify($dst, 0xf0);
                if ($update_status) {
                    test.verify(Z, false);
                    test.verify(N, true);
                }
            }
        }
    };
}

lazy_static! {
static ref INSTRUCTIONS: Vec<InstructionInfo> =  vec![
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
    instruction_info!(brk),
    instruction_info!(bvc),
    instruction_info!(bvs),
    instruction_info!(clc),
    instruction_info!(cld),
    instruction_info!(cli),
    instruction_info!(clv),
    instruction_info!(cmp),
    instruction_info!(cpx),
    instruction_info!(cpy),
    instruction_info!(dec),
    instruction_info!(dex),
    instruction_info!(dey),
    instruction_info!(eor),
    instruction_info!(inc),
    instruction_info!(inx),
    instruction_info!(iny),
    instruction_info!(jmp),
    instruction_info!(jsr),
    instruction_info!(lda),
    instruction_info!(ldx),
    instruction_info!(ldy),
    instruction_info!(lsr),
    instruction_info!(nop),
    instruction_info!(ora),
    instruction_info!(pha),
    instruction_info!(php),
    instruction_info!(pla),
    instruction_info!(plp),
    instruction_info!(rol),
    instruction_info!(ror),
    instruction_info!(rti),
    instruction_info!(rts),
    instruction_info!(sbc),
    instruction_info!(sec),
    instruction_info!(sed),
    instruction_info!(sei),
    instruction_info!(sta),
    instruction_info!(stx),
    instruction_info!(sty),
    instruction_info!(tax),
    instruction_info!(tay),
    instruction_info!(tsx),
    instruction_info!(txa),
    instruction_info!(txs),
    instruction_info!(tya),
    // unofficial instructions
    instruction_info!(slo),
    instruction_info!(sre),
    instruction_info!(lax),
    instruction_info!(lar),
    instruction_info!(kil),
    instruction_info!(isc),
    instruction_info!(dcp),
    instruction_info!(axs),
    instruction_info!(sax),
];

pub static ref INST_FACTORIES_BY_OP_CODE: HashMap<u8, InstFactory> = {
    let mut inst_factory_by_op_code: HashMap<u8, InstFactory> = HashMap::new();
    for info in INSTRUCTIONS.iter() {
        for (op, mode) in info.opcode_to_addressing_mode {
            let res = inst_factory_by_op_code.insert(
                *op,
                InstFactory {
                    opcode: *op,
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

pub static ref INST_FACTORIES_BY_NAME_MODE: HashMap<(String, AddressingMode), InstFactory> = {
    let mut inst_factory_by_name_mode: HashMap<(String, AddressingMode), InstFactory> = HashMap::new();
    for info in INSTRUCTIONS.iter() {
        for (op, mode) in info.opcode_to_addressing_mode {
            let res = inst_factory_by_name_mode.insert(
                (info.name.clone(), mode.clone()),
                InstFactory {
                    opcode: *op,
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
    inst_factory_by_name_mode
};
}

pub struct InstIter {
    bytes: Vec<u8>,
    idx: usize,
}

impl InstIter {
    pub fn new(bytes: &[u8]) -> Self {
        InstIter {
            bytes: bytes.to_vec(),
            idx: 0,
        }
    }
}

impl Iterator for InstIter {
    type Item = Inst;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.bytes.len() {
            return None;
        }
        let op = self.bytes[self.idx];
        if let Some(factory) = INST_FACTORIES_BY_OP_CODE.get(&op) {
            let inst = factory.make(&self.bytes[(self.idx + 1)..]);
            self.idx += inst.len() as usize;
            Some(inst)
        } else {
            panic!("unknown op code: {:#x}", op);
        }
    }
}

pub fn disassemble(bytes: &[u8]) -> InstIter {
    InstIter::new(bytes)
}

pub fn adc_helper(b: u8, cpu: &mut CPU) {
    let result16 = cpu.a as u16 + b as u16 + cpu.flags.c() as u16;
    let result = result16 as u8;
    cpu.flags.set_c((result16 >> 8) & 1 != 0);
    cpu.flags.set_v((cpu.a ^ result) & (b ^ result) & 0x80 != 0);
    cpu.a = result;
    cpu.update_z(cpu.a);
    cpu.update_n(cpu.a);
}