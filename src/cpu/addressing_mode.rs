use crate::cpu::CPU;

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,
    IndirectIndexed,
    Relative,
}

impl AddressingMode {

    pub fn read_param(&self,  bytes: &[u8]) -> Option<u16> {
        match *self {
            AddressingMode::Implied |
            AddressingMode::Accumulator => None,
            AddressingMode::Immediate
            | AddressingMode::Relative
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => Some(bytes[0] as u16),
            AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
                let lsb: u16 = bytes[0] as u16;
                let msb: u16 = bytes[1] as u16;
                Some((msb << 8) + lsb)
            }
        }
    }
}


pub fn load_operand_opt(mode: AddressingMode, cpu: &CPU, param: Option<u16>) -> u8 {
    match mode {
        AddressingMode::Accumulator => return cpu.a,
        _ => load_operand(mode, cpu, param.unwrap())
    }
}

pub fn load_operand(mode: AddressingMode, cpu: &CPU, param: u16) -> u8 {
    match mode {
        AddressingMode::Implied => {
            panic!("load_operand should not be called for Implied instruction")
        }
        AddressingMode::Accumulator => cpu.a,
        AddressingMode::Relative |
        AddressingMode::Immediate => {
            assert!(param <= 0xff);
            param as u8
        }
        AddressingMode::ZeroPage => {
            assert!(param <= 0xff);
            cpu.mem[param as usize]
        }
        AddressingMode::ZeroPageX => {
            assert!(param <= 0xff);
            cpu.mem[(cpu.x.wrapping_add(param as u8)) as usize]
        }
        AddressingMode::Absolute => cpu.mem[param as usize],
        AddressingMode::AbsoluteX => cpu.mem[(param.wrapping_add(cpu.x as u16)) as usize],
        AddressingMode::AbsoluteY => cpu.mem[(param.wrapping_add(cpu.y as u16)) as usize],
        AddressingMode::IndexedIndirect => {
            assert!(param <= 0xff);
            let addr = (cpu.x.wrapping_add(param as u8)) as usize;
            let addr = cpu.get_mem16(addr) as usize;
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
