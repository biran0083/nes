use crate::define_ld_inst;

define_ld_inst!(Y, &[
    (0xA0, AddressingMode::Immediate),
    (0xA4, AddressingMode::ZeroPage),
    (0xB4, AddressingMode::ZeroPageX),
    (0xAC, AddressingMode::Absolute),
    (0xBC, AddressingMode::AbsoluteX),
]);