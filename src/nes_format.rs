use crate::io::read_file;

// https://formats.kaitai.io/ines/index.html
pub struct NesFile {
    pub header: NesHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub title: Option<String>,
}

pub struct NesHeader {
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub flags_6: u8,
    pub flags_7: u8,
    pub prg_ram_size: u8,
    pub flags_9: u8,
    pub flags_10: u8,
}

pub fn read_nes_file(file_path: &str) -> Result<NesFile, std::io::Error> {
    let buffer = read_file(file_path)?;
    let mut b = buffer.as_slice();
    let header = parse_nes_header(&b[0..16]);
    b = &b[16..];
    let prg_rom = b[0..header.prg_rom_size as usize * 0x4000].to_vec();
    b = &b[prg_rom.len()..];
    let chr_rom = b[0..header.chr_rom_size as usize * 0x2000].to_vec();
    b = &b[chr_rom.len()..];
    // TODO: handle player choice
    let title = if b.len() > 8224 {
        Some(b[8224..].iter().map(|&c| c as char).collect())
    } else {
        None
    };
    Ok(NesFile {
        header,
        prg_rom,
        chr_rom,
        title,
    })
}

fn parse_nes_header(buffer: &[u8]) -> NesHeader {
    NesHeader {
        prg_rom_size: buffer[4],
        chr_rom_size: buffer[5],
        flags_6: buffer[6],
        flags_7: buffer[7],
        prg_ram_size: buffer[8],
        flags_9: buffer[9],
        flags_10: buffer[10],
    }
}