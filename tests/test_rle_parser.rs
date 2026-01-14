use game_of_life::rle_parser::RLEParser;
use game_of_life::read_file_to_string;

#[cfg(test)]
mod tests {
    use super::*;
    static FILE_CONTENTS: &str = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";

    #[test]
    fn test_read_file_to_string() {
        let result = read_file_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, World!\n");
    }

    #[test]
    fn test_read_rle_file_to_string() {
        let result = read_file_to_string("test_data/glider.rle").unwrap();
        assert_eq!(result, FILE_CONTENTS)
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
