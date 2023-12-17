use nes::{
    addressing_mode::AddressingMode,
    instructions::{make_brk, make_inx, make_lda, make_tax, InstFactory},
};
use std::collections::HashMap;

fn main() {
    let bytes: Vec<u8> = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    let mut iter = bytes.iter();
    let mut inst_factory_by_op_code: HashMap<u8, InstFactory> = HashMap::new();
    for (opcode, mode) in [
        (0xa9, AddressingMode::Immediate),
        (0x05, AddressingMode::ZeroPage),
        (0x15, AddressingMode::ZeroPageX),
        (0x0D, AddressingMode::Absolute),
        (0x1D, AddressingMode::AbsoluteX),
        (0x19, AddressingMode::AbsoluteY),
        (0x01, AddressingMode::IndexedIndirect),
        (0x11, AddressingMode::IndirectIndexed),
    ] {
        inst_factory_by_op_code.insert(opcode, InstFactory { mode, f: make_lda });
    }
    inst_factory_by_op_code.insert(
        0xaa,
        InstFactory {
            mode: AddressingMode::Implied,
            f: make_tax,
        },
    );
    inst_factory_by_op_code.insert(
        0xe8,
        InstFactory {
            mode: AddressingMode::Implied,
            f: make_inx,
        },
    );
    inst_factory_by_op_code.insert(
        0x00,
        InstFactory {
            mode: AddressingMode::Implied,
            f: make_brk,
        },
    );
    while let Some(b) = iter.next() {
        if let Some(factory) = inst_factory_by_op_code.get(b) {
            let inst = factory.make(&mut iter);
            println!("{inst:?}");
        } else {
            panic!("unknown op code: {:#x}", b);
        }
    }
}
