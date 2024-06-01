use std::{fmt::Debug, str::FromStr};

use crate::{error::NesError, instructions::{Inst, INST_FACTORIES_BY_OP_CODE}};
use error_stack::{bail, Result};
use thiserror::Error;


/**

 7  bit  0
---- ----
NV1B DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
|||+------ (No CPU effect; see: the B flag)
||+------- (No CPU effect; always pushed as 1)
|+-------- Overflow
+--------- Negative
 */
#[derive(Clone, PartialEq)]
pub struct Flags {
    pub value: u8,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            value: 0b0010_0100,
        }
    }

}
impl Flags {
    fn get_bit(&self, n: u8) -> bool {
        self.value & (1 << n) != 0
    }

    fn set_bit(&mut self, n: u8, v: bool) {
        if v {
            self.value |= 1 << n;
        } else {
            self.value &= !(1 << n);
        }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, v: u8) {
        self.value = v | 0b0010_0000;
    }

    pub fn c(&self) -> bool {
        self.get_bit(0)
    }

    pub fn set_c(&mut self, c: bool) {
        self.set_bit(0, c)
    }

    pub fn z(&self) -> bool {
        self.get_bit(1)
    }

    pub fn set_z(&mut self, c: bool) {
        self.set_bit(1, c)
    }

    pub fn i(&self) -> bool {
        self.get_bit(2)
    }

    pub fn set_i(&mut self, i: bool) {
        self.set_bit(2, i)
    }

    pub fn d(&self) -> bool {
        self.get_bit(3)
    }

    pub fn set_d(&mut self, d: bool) {
        self.set_bit(3, d)
    }

    pub fn b(&self) -> bool {
        self.get_bit(4)
    }

    pub fn set_b(&mut self, b: bool) {
        self.set_bit(4, b)
    }

    pub fn v(&self) -> bool {
        self.get_bit(6)
    }

    pub fn set_v(&mut self, v: bool) {
        self.set_bit(6, v)
    }

    pub fn n(&self) -> bool {
        self.get_bit(7)
    }

    pub fn set_n(&mut self, n: bool) {
        self.set_bit(7, n)
    }
}

pub struct CPU {
    // registers
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub sp: u8,
    pub pc: u16,
    pub flags: Flags,
    pub halt: bool,
    mem: Vec<u8>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            a: 0,
            sp: 0,
            pc: 0,
            flags: Flags::default(),
            halt: false,
            mem: vec![0; 0x10000],
        }
    }


    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.a = 0;
        self.sp = 0xFD;
        self.flags = Flags::default();
        self.halt = false;
        self.pc = self.get_mem16(0xFFFC);
    }

    fn get_phisical_addr(&self, addr: u16) -> u16 {
        match addr {
            0x0000..=0x1FFF => {
                addr & 0x07FF
            }
            0x2000..=0x3FFF => {
                addr & 0x2007
            }
            _ => {
                addr
            }
        }
    }

    pub fn set_mem(&mut self, addr: u16, value: u8) {
        let i = self.get_phisical_addr(addr) as usize;
        self.mem[i] = value;
    }

    pub fn set_mem16(&mut self, addr: u16, value: u16) {
        let lsb = (value & 0xff) as u8;
        let msb = (value >> 8) as u8;
        self.set_mem(addr, lsb);
        self.set_mem(addr.wrapping_add(1), msb);
    }

    pub fn get_mem(&self, addr: u16) -> u8 {
        if self.halt {
            return 0xff;
        }
        self.mem[self.get_phisical_addr(addr) as usize]
    }

    pub fn get_mem16(&self, addr: u16) -> u16 {
        let lsb = self.get_mem(addr) as u16;
        let msb = self.get_mem(addr.wrapping_add(1)) as u16;
        (msb << 8) + lsb
    }

    pub fn update_z(&mut self, value: u8) {
        self.flags.set_z(value == 0);
    }

    pub fn update_n(&mut self, value: u8) {
        self.flags.set_n(value & 0x80 != 0);
    }

    pub fn load_program(&mut self, bytes: &[u8], start: u16) {
        // The memcopy below may overwrite the start address, which is expected.
        self.set_mem16(0xFFFC, start);
        let i = start as usize;
        self.mem[i..(i + bytes.len())].copy_from_slice(bytes);
        self.pc = start;
        self.sp = 0xff;
    }

    fn decode(&self) -> Result<Inst, NesError> {
        let op = self.mem[self.pc as usize];
        if let Some(factory) = INST_FACTORIES_BY_OP_CODE
            .get(&op) {
            return Ok(factory.make(&self.mem[((self.pc + 1) as usize)..]))
        }
        bail!(NesError::DisassemblerFailure(format!("Instruction not found. opcode={:02x} pc={:04x}", op, self.pc)))
    }

    pub fn run_once(&mut self) -> Result<(), NesError> {
        if self.halt {
            bail!(NesError::HaltError{});
        }
        let ins = self.decode()?;
        tracing::debug!("[pc={:04x}] running {:?}", self.pc, ins);
        ins.run(self);
        Ok(())
    }

    pub fn run_with_callback<F>(&mut self, mut f: F) -> Result<(), NesError>
        where F: FnMut(&mut CPU) -> Result<(), NesError> {
        while !self.halt {
            f(self)?;
            self.run_once()?;
        }
        bail!(NesError::HaltError{});
    }

    pub fn get_stack_top_addr(&self) -> u16 {
        0x100 + self.sp as u16
    }

    pub fn push8(&mut self, value: u8) {
        let addr = self.get_stack_top_addr();
        self.mem[addr as usize] = value;
        self.sp -= 1;
    }

    pub fn push16(&mut self, value: u16) {
        let lsb = (value & 0xff) as u8;
        let msb = (value >> 8) as u8;
        self.push8(msb);
        self.push8(lsb);
    }

    pub fn pop8(&mut self) -> u8 {
        self.sp += 1;
        self.mem[self.get_stack_top_addr() as usize]
    }

    pub fn pop16(&mut self) -> u16 {
        let lsb = self.pop8() as u16;
        let msb = self.pop8() as u16;
        (msb << 8) + lsb
    }

    pub fn trace(&self) -> Result<CpuState, NesError> {
        let inst = self.decode()?;
        Ok(CpuState {
            x: self.x,
            y: self.y,
            a: self.a,
            sp: self.sp,
            pc: self.pc,
            flags: self.flags.clone(),
            inst,
            inst_details: None,
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct CpuState {
    x: u8,
    y: u8,
    a: u8,
    sp: u8,
    pc: u16,
    flags: Flags,
    inst: Inst,
    inst_details: Option<String>,
}

impl Debug for CpuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inst_bytes = self.inst.to_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");
        // D10E  C1 80     CMP ($80,X) @ 80 = 0200 = 80    A:80 X:00 Y:69 P:A5 SP:FB
        write!(f, "{:04X}  {:<8}  {:<11?} {:<19} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.pc, inst_bytes, self.inst, self.inst_details.clone().unwrap_or_default(), self.a, self.x, self.y, self.flags.get(), self.sp)
    }
}


#[derive(Error, Debug)]
pub enum CpuStateParseError {
    #[error("Illegal input: {0}")]
    IllegalInput(String),
    #[error("Parse error: {0}")]
    ParseIntError(String),
    #[error("Unknown opcode: {0}")]
    UnknownOpCode(u8),
    #[error("Bad instruction string: expect={expect}, actual={actual}")]
    BadInstructionString{expect: String, actual: String},
}

impl FromStr for CpuState {
    type Err = CpuStateParseError;
    // D10E  C1 80     CMP ($80,X) @ 80 = 0200 = 80    A:80 X:00 Y:69 P:A5 SP:FB
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let mut parts = parts.as_slice();
        if parts.len() < 10 {
            return Err(CpuStateParseError::IllegalInput(s.to_string()));
        }
        let pc = u16::from_str_radix(parts[0], 16)
            .map_err(|_| CpuStateParseError::ParseIntError(parts[0].to_string()))?;
        parts = &parts[1..];
        let opcode = u8::from_str_radix(parts[0], 16)
            .map_err(|_| CpuStateParseError::ParseIntError(parts[0].to_string()))?;
        let factory = INST_FACTORIES_BY_OP_CODE.get(&opcode).ok_or_else(|| CpuStateParseError::UnknownOpCode(opcode))?;
        let len = factory.mode.get_inst_size() as usize;
        let inst_bytes = parts[..len].iter()
            .map(|b| u8::from_str_radix(b, 16).
                map_err(|_| CpuStateParseError::ParseIntError(b.to_string())))
            .collect::<std::result::Result<Vec<u8>, _>>()?;
        let inst = factory.make(&inst_bytes[1..len]);
        let inst_bytes = parts[..len as usize].iter()
                .map(|b|u8::from_str_radix(b, 16)
                    .map_err(|_| CpuStateParseError::ParseIntError(b.to_string())))
                .collect::<std::result::Result<Vec<u8>, _>>()?;
        if inst_bytes != inst.to_bytes() {
            return Err(CpuStateParseError::IllegalInput(
                format!("Bad instruction string: inst={:?} expect={:?}, actual={:?}", inst, inst.to_bytes(), inst_bytes)));
        }
        parts = &parts[len..];
        let inst_str = inst.to_string(Some(pc));
        let inst_str_parts = inst_str.split_whitespace().collect::<Vec<&str>>();
        if &parts[..inst_str_parts.len()] != inst_str_parts.as_slice() {
            return Err(CpuStateParseError::BadInstructionString{
                expect: inst_str_parts.join(" "),
                actual: parts[..inst_str_parts.len()].join(" ")});
        }
        parts = &parts[inst_str_parts.len()..];
        let mut res = CpuState {
            x: 0,
            y: 0,
            a: 0,
            sp: 0,
            pc,
            flags: Flags::default(),
            inst,
            inst_details: None,
        };

        for part in parts.iter() {
            let p = part.split(":").collect::<Vec<&str>>();
            if p.len() == 2 {
                let key = p[0];
                if key == "PPU" {
                    break; // TODO: handle PPU
                }
                let value = u8::from_str_radix(p[1], 16)
                    .map_err(|_| CpuStateParseError::ParseIntError(p[1].to_string()))?;
                match key {
                    "A" => res.a = value,
                    "X" => res.x = value,
                    "Y" => res.y = value,
                    "P" => res.flags.value = value,
                    "SP" => res.sp = value,
                    _ => return Err(CpuStateParseError::IllegalInput(format!("Unknown register: {}", key))),
                }
            }
        }
        Ok(res)
    }
}