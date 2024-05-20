use crate::cpu::CPU;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,
    IndirectIndexed,
    Relative,
    Indirect,
}

impl AddressingMode {
    pub fn get_inst_size(&self) -> u16 {
        match *self {
            AddressingMode::Implied |
            AddressingMode::Accumulator => 1,
            AddressingMode::Immediate |
            AddressingMode::ZeroPage |
            AddressingMode::ZeroPageX |
            AddressingMode::ZeroPageY |
            AddressingMode::IndexedIndirect |
            AddressingMode::IndirectIndexed |
            AddressingMode::Relative => 2,
            AddressingMode::Absolute |
            AddressingMode::AbsoluteX |
            AddressingMode::AbsoluteY |
            AddressingMode::Indirect => 3,
        }
    }

    pub fn read_param(&self,  bytes: &[u8]) -> Option<u16> {
        match *self {
            AddressingMode::Implied |
            AddressingMode::Accumulator => None,
            AddressingMode::Immediate
            | AddressingMode::Relative
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => Some(bytes[0] as u16),
            AddressingMode::Absolute
            | AddressingMode::Indirect
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY => {
                let lsb: u16 = bytes[0] as u16;
                let msb: u16 = bytes[1] as u16;
                Some((msb << 8) + lsb)
            }
        }
    }
}

pub fn load_operand(mode: AddressingMode, cpu: &CPU, param: u16) -> u8 {
    match mode {
        AddressingMode::Indirect |
        AddressingMode::Implied => {
            panic!("load_operand should not be called for Implied instruction")
        }
        AddressingMode::Accumulator => cpu.a,
        AddressingMode::Relative |
        AddressingMode::Immediate => {
            assert!(param <= 0xff);
            param as u8
        }
        AddressingMode::ZeroPage |
        AddressingMode::ZeroPageX |
        AddressingMode::ZeroPageY |
        AddressingMode::Absolute |
        AddressingMode::AbsoluteX |
        AddressingMode::AbsoluteY |
        AddressingMode::IndexedIndirect => {
            let addr = load_operand_addr(mode, cpu, param);
            cpu.mem[addr]
        }
        AddressingMode::IndirectIndexed => {
            assert!(param <= 0xff);
            let addr = param as usize;
            let addr = cpu.get_mem16(addr) as usize;
            cpu.mem[addr].wrapping_add(cpu.y)
        }
    }
}

pub fn load_operand_addr(mode: AddressingMode, cpu: &CPU, param: u16) -> usize {
    match mode {
        AddressingMode::Accumulator|
        AddressingMode::Relative |
        AddressingMode::Immediate |
        AddressingMode::IndirectIndexed |
        AddressingMode::Implied => {
            panic!("load_operand_addr should not be called for {:?} instruction", mode)
        }
        AddressingMode::Indirect => {
            cpu.get_mem16(param as usize) as usize
        }
        AddressingMode::ZeroPage => {
            assert!(param <= 0xff);
            param as usize
        }
        AddressingMode::ZeroPageX => {
            assert!(param <= 0xff);
            (cpu.x.wrapping_add(param as u8)) as usize
        }
        AddressingMode::ZeroPageY => {
            assert!(param <= 0xff);
            (cpu.y.wrapping_add(param as u8)) as usize
        }
        AddressingMode::Absolute => param as usize,
        AddressingMode::AbsoluteX => (param.wrapping_add(cpu.x as u16)) as usize,
        AddressingMode::AbsoluteY => (param.wrapping_add(cpu.y as u16)) as usize,
        AddressingMode::IndexedIndirect => {
            assert!(param <= 0xff);
            let addr = (cpu.x.wrapping_add(param as u8)) as usize;
            cpu.get_mem16(addr) as usize
        }
    }
}