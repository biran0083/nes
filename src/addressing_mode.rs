use crate::cpu::CPU;

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,
    IndirectIndexed,
}

pub fn read_param(mode: AddressingMode, bytes: &[u8]) -> Option<u16> {
    match mode {
        AddressingMode::Implied => None,
        AddressingMode::Immediate
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

pub fn load_operand(mode: AddressingMode, cpu: &CPU, param: u16) -> u8 {
    match mode {
        AddressingMode::Implied => {
            panic!("load_operand should not be called for Implied instruction")
        }
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
            cpu.mem[(cpu.X.wrapping_add(param as u8)) as usize]
        }
        AddressingMode::Absolute => cpu.mem[param as usize],
        AddressingMode::AbsoluteX => cpu.mem[(param.wrapping_add(cpu.X as u16)) as usize],
        AddressingMode::AbsoluteY => cpu.mem[(param.wrapping_add(cpu.Y as u16)) as usize],
        AddressingMode::IndexedIndirect => {
            assert!(param <= 0xff);
            let addr = (cpu.X.wrapping_add(param as u8)) as usize;
            let lsb = cpu.mem[addr] as u16;
            let msb = cpu.mem[addr + 1] as u16;
            cpu.mem[((msb << 8) + lsb) as usize]
        }
        AddressingMode::IndirectIndexed => {
            assert!(param <= 0xff);
            let addr = param as usize;
            let lsb = cpu.mem[addr] as u16;
            let msb = cpu.mem[addr + 1] as u16;
            cpu.mem[((msb << 8) + lsb) as usize].wrapping_add(cpu.Y)
        }
    }
}
