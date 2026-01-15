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

pub fn parse_cmd_arguments(args: Vec<String>) -> Result<(u8, String), String> {
    if args.len() != 3 {
        let error_msg = "Invalid number of arguments.".to_string();
        print_usage();
        return Err(error_msg);
    }
    let generations: u8 = args[1].parse().unwrap();
    let file_name = args[2].clone();
    Ok((generations, file_name))
}

pub fn print_usage() -> String {
    let msg = "Usage: ./game_of_life <generations> <input_file.rle>".to_string();
    println!("{}", msg);
    msg
}
