use std::io::Read;
use std::fs::File;
use std::io::BufReader;

pub mod rle_parser;
pub mod grid;
pub mod game_of_life;



pub fn read_file_to_string(input_file: &str) -> std::io::Result<String> {
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut file_contents = String::new();
    buf_reader.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}
