#[derive(Debug, Default)]
pub struct CPU {
    // registers
    pub X: u8,
    pub Y: u8,
    pub A: u8,
    pub SP: u8,
    pub PC: u16,
    pub status: u8,
    pub mem: Vec<u8>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            mem: vec![0; 0x10000],
            ..Default::default()
        }
    }
}
