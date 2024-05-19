mod instructions;
mod cpu;
use std::fs::File;
use clap::{Arg, Command};
use instructions::disassemble;
use error_stack::Result;
use std::io::Read;

fn read_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn run_file(file: &str) -> Result<(), std::io::Error> {
    let game_code = read_file(file)?;
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(&game_code, 0x6000);
    Ok(())
}

fn disassemble_file(file: &str) -> Result<(), std::io::Error> {
    let game_code = read_file(file)?;
    for ins in disassemble(&game_code) {
        println!("{:?}", ins);
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error>{
    let matches = Command::new("NES Emulator")
        .version("1.0")
        .author("Ran Bi")
        .about("Handles running and disassembling files")
        .subcommand(
            Command::new("run")
                .about("Runs the specified file")
                .arg(Arg::new("file")
                    .help("The file to run")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("disassemble")
                .about("Disassembles the specified file")
                .arg(Arg::new("file")
                    .help("The file to disassemble")
                    .required(true)
                    .index(1))
        )
        .get_matches();
    match matches.subcommand() {
        Some(("run", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            run_file(file)?;
        },
        Some(("disassemble", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            disassemble_file(file)?;
        },
        _ => unreachable!("The CLI parser ensures that a subcommand is used"),
    }
    Ok(())
}