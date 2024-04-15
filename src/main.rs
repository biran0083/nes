mod instructions;
mod cpu;
fn main() {
    let bytes: Vec<u8> = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    for inst in  instructions::disassemble(&bytes) {
        println!("{:?}", inst);
    }
}
