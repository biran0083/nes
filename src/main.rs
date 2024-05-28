mod instructions;
mod cpu;
mod assembler;
mod error;
mod io;
mod nes_format;

use assembler::AsmLine;
use clap::{arg, Command};
use assembler::Assembler;
use cpu::CpuState;
use cpu::CPU;
use error::NesError;
use error_stack::bail;
use instructions::disassemble;
use error_stack::{Result, ResultExt};
use io::read_file;
use io::read_file_lines;
use io::write_file;
use nes_format::read_nes_file;
use rand::Rng;
use tracing::Level;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;


fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                cpu.set_mem(0xff, 0x77);
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                cpu.set_mem(0xff, 0x73);
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                cpu.set_mem(0xff, 0x61);
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                cpu.set_mem(0xff, 0x64);
            }
            _ => {/* do nothing */}
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
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32).unwrap();
    let mut cpu = cpu::CPU::new();
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
    let asm_lines = lines.iter().map(|s| s.parse::<AsmLine>()).collect::<std::result::Result<Vec<AsmLine>, NesError>>()?;
    let bytes = assembler.assemble(&asm_lines)?;
    Ok(bytes.to_vec())
}

fn parse_int16(s: &str) -> Result<u16, NesError> {
    if s.starts_with("0x") || s.starts_with("0X")  {
        return u16::from_str_radix(&s[2..], 16)
            .change_context(NesError::ParseInt);
    }
    s.parse::<u16>().change_context(NesError::ParseInt)
}

fn parse_trace_log_file(fname: &str) -> Result<Vec<CpuState>, NesError> {
    let lines = read_file_lines(fname).unwrap();
    let mut states = Vec::new();
    for line in lines {
        let state = line.parse::<CpuState>().change_context(NesError::ParseCpuStateError)?;
        states.push(state)
    }
    Ok(states)
}

fn test_code(code: Vec<u8>, start_addr: u16, logs: Vec<CpuState>) -> Result<(), NesError> {
    Ok(())
}

fn main() -> Result<(), NesError>{
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let matches = Command::new("NES Emulator")
        .version("1.0")
        .author("Ran Bi")
        .about("Handles running and disassembling files")
        .subcommand(
            Command::new("run")
                .about("Runs the specified file")
                .arg(arg!(--start <ADDRESS> "The start address for assembling")
                    .default_value("0x0600")
                    .required(false))
                .arg(arg!(<FILE> "The file to run")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("disassemble")
                .about("Disassembles the specified file")
                .arg(arg!(<FILE> "The file to disassemble")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("assemble")
                .about("Assembles the specified file")
                .arg(arg!(--start <ADDRESS> "The start address for assembling")
                    .required(false))
                .arg(arg!(--out <OUT> "The output file")
                    .required(true))
                .arg(arg!(<FILE> "The file to assemble")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("test")
                .about("Run program in test mode")
                .arg(arg!(--start <ADDRESS> "The start address for assembling")
                    .required(false))
                .arg(arg!(--out <OUT> "The output for cpu traces")
                    .required(true))
                .arg(arg!(<FILE> "The file to test")
                    .required(true)
                    .index(1))
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
        },
        Some(("disassemble", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            disassemble_file(file)?;
        },
        Some(("assemble", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            let output_file = sub_m.get_one::<String>("out").unwrap();
            let start = parse_int16(sub_m.get_one::<String>("start").unwrap())
                .change_context(NesError::ParseInt)?;
            let bytes = assemble_file(file, start)?;
            write_file(output_file, &bytes).change_context(NesError::Io)?;
        },
        Some(("test", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            let trace_log_file = sub_m.get_one::<String>("out").unwrap();
            let start = parse_int16(sub_m.get_one::<String>("start").unwrap())
                .change_context(NesError::ParseInt)?;
            let logs: Vec<CpuState> = parse_trace_log_file(trace_log_file)?;
            if file.ends_with(".nes") {
                let nes_file = read_nes_file(file).change_context(NesError::Io)?;
                test_code(nes_file.prg_rom, start, logs)?;
            } else {
                bail!(NesError::InvalidFileExtension(file.to_string()));
            }
        }
        _ => unreachable!("The CLI parser ensures that a subcommand is used"),
    }
    Ok(())
}