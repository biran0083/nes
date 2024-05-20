use std::str::FromStr;

use error_stack::{Result, bail};

use crate::cpu::addressing_mode::AddressingMode;
use crate::error::NesError;
use crate::instructions::INST_FACTORIES_BY_NAME_MODE;

pub struct Assembler {
    label_addr: std::collections::HashMap<String, u16>,
    addr: u16,
    start_addr: u16,
    bytes: Vec<u8>,
}

impl Assembler {
    pub fn new(start_addr: u16) -> Self {
        Self {
            label_addr: std::collections::HashMap::new(),
            addr: start_addr,
            start_addr,
            bytes: Vec::new(),
        }
    }

    fn handle(&mut self, name: &str, mode: AddressingMode, operand: Option<u16>) -> Result<(), NesError> {

        if let Some(factory) = INST_FACTORIES_BY_NAME_MODE.get(&(name.to_string(), mode)) {
            let inst = factory.make2(operand);
            let bs = inst.to_bytes();
            self.bytes.extend_from_slice(&bs);
            self.addr += mode.get_inst_size();
            Ok(())
        } else {
            bail!(NesError::InstNotFound(format!("{}, {:?}", name.to_string(), mode)))
        }
    }

    fn parse_int8(&self, s: &str) -> Option<u8> {
        if s.starts_with("$") {
            return u8::from_str_radix(&s[1..], 16).ok()
        }
        s.parse().ok()
    }

    fn parse_int16(&self, s: &str) -> Option<u16> {
        if s.starts_with("$") {
            return u16::from_str_radix(&s[1..], 16).ok()
        }
        s.parse().ok()
    }

    fn get_operand_value(&self, line: &AsmLine) -> Result<Option<u16>, NesError>  {
        match line {
            AsmLine::Empty => Ok(None),
            AsmLine::Label{name: _} => Ok(None),
            AsmLine::Inst1{name: _, mode: _} => Ok(None),
            AsmLine::Inst2{name: _, mode, operand} => {
                match mode {
                    AddressingMode::Implied |
                    AddressingMode::Accumulator => panic!("Illegal instruction: {:?}", line),
                    AddressingMode::Immediate|
                    AddressingMode::ZeroPage  |
                    AddressingMode::ZeroPageX |
                    AddressingMode::ZeroPageY => {
                        if let Some(operand) = self.parse_int8(operand) {
                            return Ok(Some(operand as u16));
                        }
                    },
                    AddressingMode::Relative => {
                        if let Some(operand) = self.parse_int8(operand) {
                            return Ok(Some(operand as u16));
                        } else if let Some(addr) = self.label_addr.get(operand) {
                            let target_addr  = *addr;
                            let current_addr = self.addr.wrapping_add(2);
                            let diff =  target_addr.wrapping_sub(current_addr) as i16;
                            if diff >= -128 && diff <= 127 {
                                return Ok(Some(diff as u16));
                            }
                        }
                    }
                    AddressingMode::IndexedIndirect |
                    AddressingMode::IndirectIndexed |
                    AddressingMode::Absolute |
                    AddressingMode::AbsoluteX |
                    AddressingMode::AbsoluteY => {
                        if let Some(operand) = self.parse_int16(operand) {
                            return Ok(Some(operand));
                        }
                        if let Some(addr) = self.label_addr.get(operand) {
                            return Ok(Some(*addr));
                        }
                    }
                    AddressingMode::Indirect => {
                        if let Some(operand) = self.parse_int16(&operand) {
                            return Ok(Some(operand));
                        }
                        if let Some(addr) = self.label_addr.get(operand) {
                            return Ok(Some(*addr));
                        }
                    },
                }
                bail!(NesError::AssemblerFailure(format!("illegal instruction: {:?}", line)));
            },
        }
    }

    fn assemble_line(&mut self, line: &AsmLine) -> Result<(), NesError> {
        match line {
            AsmLine::Empty |
            AsmLine::Label{name: _} => return Ok(()),
            AsmLine::Inst1{name, mode} => {
                self.handle(name, *mode, None)?;
            },
            AsmLine::Inst2{name, mode, operand: _} => {
                let operand_value = self.get_operand_value(line)?;
                self.handle(name, *mode, operand_value)?;
            },
        }
        Ok(())
    }

    fn build_label_addr(&mut self, lines: &[AsmLine]) -> Result<(), NesError> {
        self.label_addr.clear();
        let mut cur = self.start_addr;
        for line in lines {
            match line {
                AsmLine::Label{name} => {
                    self.label_addr.insert(name.to_string(), cur);
                },
                _ => {
                    cur += line.get_inst_size();
                }
            }
        }
        Ok(())
    }

    pub fn assemble(&mut self, lines: &[AsmLine]) -> Result<&[u8], NesError> {
        self.build_label_addr(lines)?;
        self.addr = self.start_addr;
        self.bytes.clear();
        for line in lines {
            self.assemble_line(line)?;
        }
        Ok(self.bytes.as_slice())
    }
}

#[derive(Debug)]
pub enum AsmLine {
    Empty,
    Label{name: String},
    Inst1{name: String, mode: AddressingMode},
    Inst2{name: String, mode: AddressingMode, operand: String},
}

impl AsmLine {
    fn get_inst_size(&self) -> u16 {
        match self {
            AsmLine::Empty => 0,
            AsmLine::Label{name: _} => 0,
            AsmLine::Inst1{name: _, mode} => mode.get_inst_size(),
            AsmLine::Inst2{name: _, mode, operand: _} => mode.get_inst_size(),
        }
    }
}

impl FromStr for AsmLine {
    type Err = NesError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut s: &str = s;
        if let Some(i) = s.find(';') {
            s = s[..i].trim();
        } else {
            s = s.trim();
        }
        if s.is_empty() {
            return Ok(AsmLine::Empty);
        }
        if s.ends_with(":") {
            let label = s[..s.len() - 1].to_string();
            return Ok(AsmLine::Label{name: label});
        } else {
            let parts = s.split(|c: char| c.is_whitespace() || c == ',').collect::<Vec<_>>();
            let name = parts[0].to_uppercase();
            if parts.len() == 1 {
                if name == "ASL" || name == "ROL" || name == "LSR" || name == "ROR" {
                    return Ok(AsmLine::Inst1 { name, mode: AddressingMode::Accumulator});
                }
                return Ok(AsmLine::Inst1 { name, mode: AddressingMode::Implied});
            }
            if parts.len() == 2 {
                let p1 = parts[1];
                if name == "BEQ" || name == "BNE" || name == "BCS" || name == "BCC" || name == "BVS" || name == "BVC" || name == "BPL" || name == "BMI" {
                    return Ok(AsmLine::Inst2 { name, mode: AddressingMode::Relative, operand: p1.to_string()});
                }
                if name == "JMP" && p1.starts_with("(") && p1.ends_with(")") {
                    let operand = p1[1..p1.len() - 1].to_string();
                    return Ok(AsmLine::Inst2 { name, mode: AddressingMode::Indirect, operand});
                }
                if p1 == "A" {
                    return Ok(AsmLine::Inst1 { name, mode: AddressingMode::Accumulator});
                }
                if p1.starts_with("#") {
                    return Ok(AsmLine::Inst2 { name, mode: AddressingMode::Immediate, operand: p1[1..].to_string()});
                }
                if p1.starts_with("$") {
                    if p1.len() == 3 {
                        return Ok(AsmLine::Inst2 { name, mode: AddressingMode::ZeroPage, operand: p1.to_string()});
                    } else if p1.len() == 5 {
                        return Ok(AsmLine::Inst2 { name, mode: AddressingMode::Absolute, operand: p1.to_string()});
                    }
                }
                // operand is a label
                return Ok(AsmLine::Inst2 { name, mode: AddressingMode::Absolute, operand: p1.to_string()});
            } else if parts.len() == 3 {
                let p1 = parts[1];
                let p2 = parts[2].to_uppercase();
                if p1.starts_with("(") && p2 == "X)" {
                    return Ok(AsmLine::Inst2 { name, mode: AddressingMode::IndexedIndirect, operand: p1[1..].to_string()});
                }
                if p1.starts_with("(") && p1.ends_with(")") && p2 == "Y" {
                    return Ok(AsmLine::Inst2 { name, mode: AddressingMode::IndirectIndexed, operand: p1[1..p1.len() - 1].to_string()});
                }
                if p1.starts_with("$") {
                    if p1.len() == 5 {
                        if p2 == "X" {
                            return Ok(AsmLine::Inst2 { name, mode: AddressingMode::AbsoluteX, operand: p1.to_string()});
                        } else if p2 == "Y" {
                            return Ok(AsmLine::Inst2 { name, mode: AddressingMode::AbsoluteY, operand: p1.to_string()});
                        }
                    } else if p1.len() == 3 {
                        if p2 == "X" {
                            return Ok(AsmLine::Inst2 { name, mode: AddressingMode::ZeroPageX, operand: p1.to_string()});
                        } else if p2 == "Y" {
                            return Ok(AsmLine::Inst2 { name, mode: AddressingMode::ZeroPageY, operand: p1.to_string()});
                        }
                    }
                }
            }
              return Err(NesError::AssemblerFailure(format!("Illegal instruction: {}", s)))
        }
    }
}
