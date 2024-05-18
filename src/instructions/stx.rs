use crate::define_st_inst;

define_st_inst!(X, &[
    (0x86, AddressingMode::ZeroPage),
    (0x96, AddressingMode::ZeroPageY),
    (0x8E, AddressingMode::Absolute),
]);