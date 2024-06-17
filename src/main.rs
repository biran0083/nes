mod assembler;
mod bus;
mod cpu;
mod error;
mod instructions;
mod io;
mod nes_format;
mod ppu;
mod screen;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

use assembler::AsmLine;
use assembler::Assembler;
use clap::{arg, Command};
use cpu::CpuState;
use cpu::CPU;
use error::NesError;
use error_stack::bail;
use error_stack::{Result, ResultExt};
use instructions::disassemble;
use instructions::INST_FACTORIES_BY_OP_CODE;
use io::read_file;
use io::read_file_lines;
use io::write_file;
use nes_format::read_nes_file;
use ppu::SYSTEM_PALLETE;
use ppu::TILE_HEIGHT;
use ppu::TILE_WIDTH;
use rand::Rng;
use screen::{ScreenState, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;
use tracing::Level;

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.set_mem(0xff, 0x77);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.set_mem(0xff, 0x73);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.set_mem(0xff, 0x61);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.set_mem(0xff, 0x64);
            }
            _ => { /* do nothing */ }
        }
    }
}

fn color(byte: u8) -> Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = cpu.get_mem(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}

fn run_code(game_code: Vec<u8>, start_addr: u16) -> Result<(), NesError> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();
    let mut cpu = cpu::CPU::default();
    cpu.load_program(&game_code, start_addr);
    cpu.reset();
    let mut screen_state = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();
    cpu.run_with_callback(|cpu| {
        handle_user_input(cpu, &mut event_pump);
        cpu.set_mem(0xfe, rng.gen_range(1..=16));
        if read_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        ::std::thread::sleep(std::time::Duration::new(0, 200_000));
        Ok(())
    })?;
    Ok(())
}

fn disassemble_file(file: &str) -> Result<(), NesError> {
    let game_code = read_file(file).change_context(NesError::Io)?;
    for ins in disassemble(&game_code) {
        println!("{:?}", ins);
    }
    Ok(())
}

fn assemble_file(file_path: &str, start_addr: u16) -> Result<Vec<u8>, NesError> {
    let mut assembler = Assembler::new(start_addr);
    let lines = read_file_lines(file_path).change_context(NesError::Io)?;
    let asm_lines = lines
        .iter()
        .map(|s| s.parse::<AsmLine>())
        .collect::<std::result::Result<Vec<AsmLine>, NesError>>()?;
    let bytes = assembler.assemble(&asm_lines)?;
    Ok(bytes.to_vec())
}

fn parse_int16(s: &str) -> Result<u16, NesError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        return u16::from_str_radix(&s[2..], 16).change_context(NesError::ParseInt);
    }
    s.parse::<u16>().change_context(NesError::ParseInt)
}

struct CpuStateReader {
    reader: BufReader<File>,
}

impl CpuStateReader {
    fn new(fname: &str) -> Result<Self, NesError> {
        let file = File::open(fname).change_context(NesError::Io)?;
        let reader = BufReader::new(file);
        Ok(CpuStateReader { reader })
    }

    fn next(&mut self) -> Result<CpuState, NesError> {
        let mut line = String::new();
        if 0 == self
            .reader
            .read_line(&mut line)
            .change_context(NesError::Io)?
        {
            bail!(NesError::EndOfFile);
        }
        line.parse::<CpuState>()
            .change_context(NesError::ParseCpuStateError)
    }
}

fn test_code(
    code: Vec<u8>,
    start_addr: u16,
    mut state_reader: CpuStateReader,
) -> Result<(), NesError> {
    let mut cpu = cpu::CPU::default();
    cpu.load_program(&code, start_addr);
    cpu.reset();
    cpu.pc = start_addr;
    let res = cpu.run_with_callback(|cpu| {
        let state = cpu.trace()?;
        match state_reader.next() {
            Ok(expected) => {
                assert_eq!(state, expected);
                tracing::info!("passed: {:?}", state);
            }
            Err(e) => {
                if let Some(e) = e.downcast_ref::<NesError>() {
                    match e {
                        NesError::EndOfFile => {
                            tracing::info!("All tests passed");
                            cpu.halt();
                        }
                        _ => {
                            bail!(NesError::TestFailed(format!("{:?}", e)));
                        }
                    }
                }
                return Ok(());
            }
        }
        Ok(())
    });
    return match &res {
        Err(report) => {
            if let Some(nes_error) = report.downcast_ref::<NesError>() {
                match nes_error {
                    NesError::HaltError => Ok(()),
                    _ => res,
                }
            } else {
                res
            }
        }
        Ok(_) => Ok(()),
    };
}

fn show_tiles(nes_file: &nes_format::NesFile) -> Result<(), NesError> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Tiles",
            (SCREEN_WIDTH * 3) as u32,
            (SCREEN_HEIGHT * 3) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(
            PixelFormatEnum::RGB24,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .unwrap();
    let mut screen_state = ScreenState::new();
    let mut base_x: usize = 0;
    let mut base_y: usize = 0;
    let padding = 1;
    for tile_data in nes_file.chr_rom.chunks(16) {
        let tile = ppu::Tile::new(tile_data);
        for x in 0..8 {
            for y in 0..8 {
                let color_idx = tile.get_pixel_index(x, y);
                let color = match color_idx {
                    0 => (255, 255, 255),
                    1 => (255, 0, 0),
                    2 => (0, 255, 0),
                    3 => (0, 0, 255),
                    _ => bail!(NesError::InvalidColorIndex(color_idx)),
                };
                screen_state.update(base_x + x as usize, base_y + y as usize, color);
            }
        }
        base_x += TILE_WIDTH + padding;
        if base_x + TILE_WIDTH >= SCREEN_WIDTH {
            base_x = 0;
            base_y += TILE_HEIGHT + padding;
        }
        if base_y + 8 >= SCREEN_HEIGHT {
            break;
        }
    }
    screen_state.update(0, 1, SYSTEM_PALLETE[1]);
    texture
        .update(None, screen_state.get_picxel_data(), SCREEN_WIDTH * 3)
        .unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn main() -> Result<(), NesError> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let matches = Command::new("NES Emulator")
        .version("1.0")
        .author("Ran Bi")
        .about("Handles running and disassembling files")
        .subcommand(
            Command::new("run")
                .about("Runs the specified file")
                .arg(
                    arg!(--start <ADDRESS> "The start address for assembling")
                        .default_value("0x0600")
                        .required(false),
                )
                .arg(arg!(<FILE> "The file to run").required(true).index(1)),
        )
        .subcommand(
            Command::new("disassemble")
                .about("Disassembles the specified file")
                .arg(
                    arg!(<FILE> "The file to disassemble")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("assemble")
                .about("Assembles the specified file")
                .arg(arg!(--start <ADDRESS> "The start address for assembling").required(false))
                .arg(arg!(--out <OUT> "The output file").required(true))
                .arg(arg!(<FILE> "The file to assemble").required(true).index(1)),
        )
        .subcommand(
            Command::new("show_tiles")
                .about("Show tiles of an nes file")
                .arg(arg!(<FILE> "The file to test").required(true).index(1)),
        )
        .subcommand(
            Command::new("test")
                .about("Run program in test mode")
                .arg(arg!(--start <ADDRESS> "The start address for assembling").required(false))
                .arg(arg!(--out <OUT> "The output for cpu traces").required(true))
                .arg(arg!(<FILE> "The file to test").required(true).index(1)),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("run", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            let start = parse_int16(sub_m.get_one::<String>("start").unwrap())
                .change_context(NesError::ParseInt)?;
            if file.ends_with(".bin") {
                let code = read_file(file).change_context(NesError::Io)?;
                run_code(code, start)?;
            } else if file.ends_with(".asm") {
                let code = assemble_file(file, start)?;
                run_code(code, start)?;
            } else if file.ends_with(".nes") {
                let nes_file = read_nes_file(file).change_context(NesError::Io)?;
                run_code(nes_file.prg_rom, start)?;
            } else {
                bail!(NesError::InvalidFileExtension(file.to_string()));
            }
        }
        Some(("disassemble", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            disassemble_file(file)?;
        }
        Some(("assemble", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            let output_file = sub_m.get_one::<String>("out").unwrap();
            let start = parse_int16(sub_m.get_one::<String>("start").unwrap())
                .change_context(NesError::ParseInt)?;
            let bytes = assemble_file(file, start)?;
            write_file(output_file, &bytes).change_context(NesError::Io)?;
        }
        Some(("show_tiles", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            let nes_file = read_nes_file(file).change_context(NesError::Io)?;
            show_tiles(&nes_file)?;
        }
        Some(("test", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            let trace_log_file = sub_m.get_one::<String>("out").unwrap();
            let start = parse_int16(sub_m.get_one::<String>("start").unwrap())
                .change_context(NesError::ParseInt)?;
            if file.ends_with(".nes") {
                let nes_file = read_nes_file(file).change_context(NesError::Io)?;
                test_code(
                    nes_file.prg_rom,
                    start,
                    CpuStateReader::new(trace_log_file)?,
                )?;
                println!("instruction count: {}", INST_FACTORIES_BY_OP_CODE.len());
                for i in 0..256 {
                    if !INST_FACTORIES_BY_OP_CODE.contains_key(&(i as u8)) {
                        println!("missing instruction: {:#02x}", i);
                    }
                }
            } else {
                bail!(NesError::InvalidFileExtension(file.to_string()));
            }
        }
        _ => unreachable!("The CLI parser ensures that a subcommand is used"),
    }
    Ok(())
}
