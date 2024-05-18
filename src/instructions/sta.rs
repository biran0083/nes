use crate::define_st_inst;

define_st_inst!(A, &[
    (0x85, AddressingMode::ZeroPage),
    (0x95, AddressingMode::ZeroPageX),
    (0x8D, AddressingMode::Absolute),
    (0x9D, AddressingMode::AbsoluteX),
    (0x99, AddressingMode::AbsoluteY),
    (0x81, AddressingMode::IndexedIndirect),
    (0x91, AddressingMode::IndirectIndexed),
]);