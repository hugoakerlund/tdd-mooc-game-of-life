use std::io::Read;
use std::fs::File;
use std::io::BufReader;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn read_file_to_string(input_file: &str) -> std::io::Result<String> {
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut file_contents = String::new();
    buf_reader.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_read_file_to_string() {
        let result = read_file_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, World!\n");
    }
}
