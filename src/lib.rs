use std::io::Read;
use std::fs::File;
use std::io::BufReader;

mod rle_parser;
use crate::rle_parser::RLEParser;

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

    #[test]
    fn test_read_rle_file_to_string() {
        let result = read_file_to_string("test_data/glider.rle").unwrap();
        let expected = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";
        assert_eq!(result, expected)
    }

    #[test]
    fn test_create_RLEParser() {
        let file_contents = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";
        let parser = RLEParser::new(file_contents).unwrap();
        assert_eq!(parser.get_file_contents(), file_contents);
    }

    #[test]
    fn test_RLEParser_parse_header() {
        let file_contents = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";
        let mut parser = RLEParser::new(file_contents).unwrap();
        parser.parse_file();
        let header = "x = 3, y = 3, rule = B3/S23\r".to_string();
        assert_eq!(parser.get_header(), header);
    }

    #[test]
    fn test_RLEParser_parse_name() {
        let file_contents = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";
        let mut parser = RLEParser::new(file_contents).unwrap();
        parser.parse_file();
        let name = "Glider\r".to_string();
        assert_eq!(parser.get_name(), name);
    }

    #[test]
    fn test_RLEParser_parse_creator() {
        let file_contents = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";
        let mut parser = RLEParser::new(file_contents).unwrap();
        parser.parse_file();
        let creator = "Richard K. Guy\r".to_string();
        assert_eq!(parser.get_creator(), creator);
    }

    #[test]
    fn test_RLEParser_parse_comments() {
        let file_contents = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";
        let mut parser = RLEParser::new(file_contents).unwrap();
        parser.parse_file();
        let comments = "The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\nwww.conwaylife.com/wiki/index.php?title=Glider\r\n".to_string();
        assert_eq!(parser.get_comments(), comments);
    }

    #[test]
    fn test_RLEParser_parse_pattern() {
        let file_contents = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";
        let mut parser = RLEParser::new(file_contents).unwrap();
        parser.parse_file();
        let pattern = "bob$2bo$3o!".to_string();
        assert_eq!(parser.get_pattern(), pattern);
    }
}
