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
    static FILE_CONTENTS: &str = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";

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
    fn test_create_rleparser() {
        let parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        assert_eq!(parser.get_file_contents(), FILE_CONTENTS);
    }

    #[test]
    fn test_rleparser_get_header() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let header = "x = 3, y = 3, rule = B3/S23\r".to_string();
        assert_eq!(parser.get_header(), header);
    }

    #[test]
    fn test_rleparser_get_name() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let name = "Glider\r".to_string();
        assert_eq!(parser.get_name(), name);
    }

    #[test]
    fn test_rleparser_get_creator() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let creator = "Richard K. Guy\r".to_string();
        assert_eq!(parser.get_creator(), creator);
    }

    #[test]
    fn test_rleparser_get_comments() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let comments = "The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\nwww.conwaylife.com/wiki/index.php?title=Glider\r\n".to_string();
        assert_eq!(parser.get_comments(), comments);
    }

    #[test]
    fn test_rleparser_get_pattern() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let pattern = "bob$2bo$3o!".to_string();
        assert_eq!(parser.get_pattern(), pattern);
    }

    #[test]
    fn test_rleparser_get_width() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let width = 3;
        assert_eq!(parser.get_width(), width);
    }

    #[test]
    fn test_rleparser_get_height() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let height = 3;
        assert_eq!(parser.get_height(), height);
    }

    #[test]
    fn test_rleparser_get_rule() {
        let mut parser = RLEParser::new(&FILE_CONTENTS).unwrap();
        parser.parse_file();
        let rule = "B3/S23\r";
        assert_eq!(parser.get_rule(), rule);
    }
}
