pub struct PPU {
    control: u8,
    mask: u8,
    status: u8,
    oma_addr: u8,
    oma_data: u8,
    scroll: u8,
    addr: u8,
    data: u8,
    oam_dma: u8,
}
