use nes::instructions::disassemble;

fn main() {
    let bytes: Vec<u8> = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    for inst in disassemble(&bytes) {
        println!("{:?}", inst);
    }
}
