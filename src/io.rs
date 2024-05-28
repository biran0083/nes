use std::{fs::File, io::{Read, Write}};


pub fn read_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.lines().map(|s| s.to_string()).collect())
}

pub fn write_file(file_path: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(bytes)?;
    Ok(())
}