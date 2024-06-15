use crate::ppu::PPU;

pub struct Bus {
    ram: [u8; 0x10000],
    ppu: PPU,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: [0; 0x10000],
            ppu: PPU::new(),
        }
    }

    fn get_phisical_addr(&self, addr: u16) -> u16 {
        match addr {
            0x0000..=0x1FFF => addr & 0x07FF,
            0x2000..=0x3FFF => addr & 0x2007,
            _ => addr,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let addr = self.get_phisical_addr(addr);
        match addr {
            0x0000..=0x7FF => self.ram[addr as usize],
            0x2000..=0x2007 | 0x4014 => self.ppu.get_ram_mapped_register(addr),
            _ => self.ram[addr as usize],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let addr = self.get_phisical_addr(addr);
        match addr {
            0x0000..=0x7FF => self.ram[addr as usize] = data,
            0x2000..=0x2007 | 0x4014 => self.ppu.set_ram_mapped_register(addr, data),
            _ => self.ram[addr as usize] = data,
        }
    }

    pub fn get_byte_stream(&self, addr: u16) -> ByteStream {
        ByteStream::new(self, addr)
    }
}

pub struct ByteStream<'a> {
    bus: &'a Bus,
    addr: u16,
}

impl<'a> ByteStream<'a> {
    fn new(bus: &'a Bus, addr: u16) -> ByteStream<'a> {
        ByteStream { bus, addr }
    }
}

impl<'a> Iterator for ByteStream<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.bus.read(self.addr);
        self.addr += 1;
        Some(result)
    }
}
