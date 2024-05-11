use crate::defube_cmp_inst;


defube_cmp_inst!(X, &[
    (0xE0, AddressingMode::Immediate),
    (0xE4, AddressingMode::ZeroPage),
    (0xEC, AddressingMode::Absolute),
]);