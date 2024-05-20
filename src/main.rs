mod instructions;
mod cpu;
mod assembler;
mod error;

use std::fs::File;
use assembler::AsmLine;
use clap::{arg, Command};
use assembler::Assembler;
use error::NesError;
use instructions::disassemble;
use error_stack::{Result, ResultExt};
use tracing::Level;
use std::io::Read;
use std::io::Write;

fn read_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn read_file_lines(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.lines().map(|s| s.to_string()).collect())
}

fn write_file(file_path: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(bytes)?;
    Ok(())
}

fn run_file(file: &str) -> Result<(), std::io::Error> {
    let game_code = read_file(file)?;
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(&game_code, 0x6000);
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

fn main() -> Result<(), NesError>{
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let matches = Command::new("NES Emulator")
        .version("1.0")
        .author("Ran Bi")
        .about("Handles running and disassembling files")
        .subcommand(
            Command::new("run")
                .about("Runs the specified file")
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
        .get_matches();
    match matches.subcommand() {
        Some(("run", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE").unwrap();
            run_file(file).change_context(NesError::Io)?;
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
        _ => unreachable!("The CLI parser ensures that a subcommand is used"),
    }
    Ok(())
}