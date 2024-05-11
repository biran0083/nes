use crate::defube_cmp_inst;


defube_cmp_inst!(A, &[
    (0xC9, AddressingMode::Immediate),
    (0xC5, AddressingMode::ZeroPage),
    (0xD5, AddressingMode::ZeroPageX),
    (0xCD, AddressingMode::Absolute),
    (0xDD, AddressingMode::AbsoluteX),
    (0xD9, AddressingMode::AbsoluteY),
    (0xC1, AddressingMode::IndexedIndirect),
    (0xD1, AddressingMode::IndirectIndexed),
]);