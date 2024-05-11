use crate::defube_cmp_inst;


defube_cmp_inst!(Y, &[
    (0xC0, AddressingMode::Immediate),
    (0xC4, AddressingMode::ZeroPage),
    (0xCC, AddressingMode::Absolute),
]);