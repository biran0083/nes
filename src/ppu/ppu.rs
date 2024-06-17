use crate::nes_format::Mirroring;

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
        self.addr = self.addr & 0x3FFF;
        self.low_byte = !self.low_byte;
    }

    pub fn get(&self) -> u16 {
        self.addr
    }

    pub fn increment(&mut self, value: u16) {
        self.addr = (self.addr + value) & 0x3fff;
    }

    pub fn get_last_written(&self) -> u8 {
        if !self.low_byte {
            (self.addr & 0x00FF) as u8
        } else {
            ((self.addr & 0xFF00) >> 8) as u8
        }
    }
}

struct PpuRegisters {
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
}

impl PpuRegisters {
    fn new() -> PpuRegisters {
        PpuRegisters {
            control: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: Default::default(),
            data: 0,
            oam_dma: 0,
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

    fn get_ppu_addr_increment(&self) -> u16 {
        if self.control & 0b00000100 == 0 {
            1
        } else {
            32
        }
    }

    pub fn increment_address(&mut self) {
        self.addr.increment(self.get_ppu_addr_increment());
    }

    pub fn get_ram_mapped_register(&self, addr: u16) -> u8 {
        match addr {
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Cannot read write only PPU address {:x}", addr)
            }
            0x2002 => self.status,
            0x2004 => self.oam_data,
            0x2007 => self.data,
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

pub struct PPU {
    registers: PpuRegisters,
    chr_rom: Vec<u8>,
    palette_table: [u8; 32],
    vram: [u8; 2048],
    oam_data: [u8; 256],
    read_buffer: u8,
    mirroring: Mirroring,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            registers: PpuRegisters::new(),
            chr_rom: Default::default(),
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],
            read_buffer: 0,
            mirroring: Mirroring::Horizontal,
        }
    }
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> PPU {
        PPU {
            registers: PpuRegisters::new(),
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],
            read_buffer: 0,
            mirroring,
        }
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let vram_index = addr - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match self.mirroring {
            Mirroring::Horizontal => match name_table {
                0 => vram_index,
                1 => vram_index - 0x400,
                2 => vram_index - 0x400,
                3 => vram_index - 0x800,
                _ => unreachable!(),
            },
            Mirroring::Vertical => match name_table {
                0 => vram_index,
                1 => vram_index,
                2 => vram_index - 0x800,
                3 => vram_index - 0x800,
                _ => unreachable!(),
            },
        }
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.registers.addr.get();
        self.registers.increment_address();
        match addr {
            0x0000..=0x1FFF => {
                let res = self.read_buffer;
                self.read_buffer = self.chr_rom[addr as usize];
                res
            }
            0x2000..=0x2FFF => {
                let res = self.read_buffer;
                self.read_buffer = self.vram[self.mirror_vram_addr(addr) as usize];
                res
            }
            0x3000..=0x3EFF => panic!("unused address space"),
            0x3F00..=0x3FFF => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("Invalid PPU address: {:#X}", addr),
        }
    }

    pub fn get_ram_mapped_register(&self, addr: u16) -> u8 {
        self.registers.get_ram_mapped_register(addr)
    }

    pub fn set_ram_mapped_register(&mut self, addr: u16, value: u8) {
        self.registers.set_ram_mapped_register(addr, value);
    }
}
