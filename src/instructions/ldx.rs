use crate::define_ld_inst;

define_ld_inst!(X, &[
    (0xA2, AddressingMode::Immediate),
    (0xA6, AddressingMode::ZeroPage),
    (0xB6, AddressingMode::ZeroPageY),
    (0xAE, AddressingMode::Absolute),
    (0xBE, AddressingMode::AbsoluteY),
]);