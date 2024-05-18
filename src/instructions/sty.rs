use crate::define_st_inst;

define_st_inst!(Y, &[
    (0x84, AddressingMode::ZeroPage),
    (0x94, AddressingMode::ZeroPageX),
    (0x8C, AddressingMode::Absolute),
]);