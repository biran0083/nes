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

pub fn read_param(mode: AddressingMode, iter: &mut std::slice::Iter<u8>) -> Option<u16> {
    match mode {
        AddressingMode::Implied => None,
        AddressingMode::Immediate
        | AddressingMode::ZeroPage
        | AddressingMode::ZeroPageX
        | AddressingMode::IndexedIndirect
        | AddressingMode::IndirectIndexed => Some(*iter.next().unwrap() as u16),
        AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
            let lsb: u16 = *iter.next().unwrap() as u16;
            let msb: u16 = *iter.next().unwrap() as u16;
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
            cpu.mem[(cpu.X + (param as u8)) as usize]
        }
        AddressingMode::Absolute => cpu.mem[param as usize],
        AddressingMode::AbsoluteX => cpu.mem[(param + cpu.X as u16) as usize],
        AddressingMode::AbsoluteY => cpu.mem[(param + cpu.Y as u16) as usize],
        AddressingMode::IndexedIndirect => {
            assert!(param <= 0xff);
            let addr = cpu.mem[(cpu.X as u16 + param) as usize] as usize;
            let lsb = cpu.mem[addr] as u16;
            let msb = cpu.mem[addr + 1] as u16;
            cpu.mem[((msb << 8) + lsb) as usize]
        }
        AddressingMode::IndirectIndexed => {
            assert!(param <= 0xff);
            let addr = cpu.mem[param as usize] as usize;
            let lsb = cpu.mem[addr] as u16;
            let msb = cpu.mem[addr + 1] as u16;
            cpu.mem[((msb << 8) + lsb) as usize] + cpu.Y
        }
    }
}
