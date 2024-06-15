struct AddressRegister {
    addr: u16,
    low_byte: bool,
}

impl Default for AddressRegister {
    fn default() -> Self {
        AddressRegister {
            addr: 0,
            low_byte: false,
        }
    }
}

impl AddressRegister {
    pub fn set(&mut self, value: u8) {
        if self.low_byte {
            self.addr = (self.addr & 0xFF00) | value as u16;
        } else {
            self.addr = (self.addr & 0x00FF) | ((value as u16) << 8);
        }
        self.low_byte = !self.low_byte;
    }

    pub fn get(&self) -> u16 {
        self.addr
    }

    pub fn increment(&mut self, value: u8) {
        self.addr = self.addr.wrapping_add(value as u16);
    }

    pub fn get_last_written(&self) -> u8 {
        if !self.low_byte {
            (self.addr & 0x00FF) as u8
        } else {
            ((self.addr & 0xFF00) >> 8) as u8
        }
    }
}

pub struct PPU {
    /**
        7  bit  0
        ---- ----
        VPHB SINN
        |||| ||||
        |||| ||++- Base nametable address
        |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
        |||| |+--- VRAM address increment per CPU read/write of PPUDATA
        |||| |     (0: add 1, going across; 1: add 32, going down)
        |||| +---- Sprite pattern table address for 8x8 sprites
        ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
        |||+------ Background pattern table address (0: $0000; 1: $1000)
        ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
        |+-------- PPU master/slave select
        |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
        +--------- Generate an NMI at the start of the
                vertical blanking interval (0: off; 1: on)
    */
    control: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    oam_data: u8,
    scroll: u8,
    addr: AddressRegister,
    data: u8,
    oam_dma: u8,
    ram: [u8; 0xffff],
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            control: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: Default::default(),
            data: 0,
            oam_dma: 0,
            ram: [0; 0xffff],
        }
    }

    pub fn get_background_table_address(&self) -> u16 {
        if self.control & 0b00010000 == 0 {
            0x0000
        } else {
            0x1000
        }
    }

    pub fn get_sprite_table_address(&self) -> u16 {
        if self.control & 0b00001000 == 0 {
            0x0000
        } else {
            0x1000
        }
    }

    pub fn get_name_table_address(&self) -> u16 {
        match self.control & 3 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => unreachable!(),
        }
    }

    pub fn get_ppu_addr_increment(&self) -> u16 {
        if self.control & 0b00000100 == 0 {
            1
        } else {
            32
        }
    }

    pub fn get_ram_mapped_register(&self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.control,
            0x2001 => self.mask,
            0x2002 => self.status,
            0x2003 => self.oam_addr,
            0x2004 => self.oam_data,
            0x2005 => self.scroll,
            0x2006 => self.addr.get_last_written(),
            0x2007 => self.data,
            0x4014 => todo!("implement OAM DAM"),
            _ => panic!("Invalid PPU register address: {:#X}", addr),
        }
    }

    pub fn set_ram_mapped_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 => self.control = value,
            0x2001 => self.mask = value,
            0x2002 => self.status = value,
            0x2003 => self.oam_addr = value,
            0x2004 => self.oam_data = value,
            0x2005 => self.scroll = value,
            0x2006 => self.addr.set(value),
            0x2007 => self.data = value,
            0x4014 => self.oam_dma = value,
            _ => panic!("Invalid PPU register address: {:#X}", addr),
        }
    }
}
