use crate::define_ld_inst;

define_ld_inst!(A, &[
    (0xA9, AddressingMode::Immediate),
    (0xA5, AddressingMode::ZeroPage),
    (0xB5, AddressingMode::ZeroPageX),
    (0xAD, AddressingMode::Absolute),
    (0xBD, AddressingMode::AbsoluteX),
    (0xB9, AddressingMode::AbsoluteY),
    (0xA1, AddressingMode::IndexedIndirect),
    (0xB1, AddressingMode::IndirectIndexed),
]);