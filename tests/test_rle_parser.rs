use game_of_life::read_file_to_string;
use game_of_life::rle_parser::RLEParser;

#[cfg(test)]
mod tests {
    use super::*;
    static GLIDER_FILE_CONTENTS: &str = "#N Glider\r\n#O Richard K. Guy\r\n#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\n#C www.conwaylife.com/wiki/index.php?title=Glider\r\nx = 3, y = 3, rule = B3/S23\r\nbob$2bo$3o!";

    static WORM_FILE_CONTENTS: &str = "#N Worm\r\n#O ?\r\n#C An 11-bit induction coil.\r\n#C https://conwaylife.com/wiki/Worm\r\nx = 6, y = 4, rule = B3/S23\r\nb2o$obo$o4bo$b5o!\r\n";


    #[test]
    fn test_read_rle_file_to_string() {
        let result = read_file_to_string("test_data/glider.rle").unwrap();
        assert_eq!(result, GLIDER_FILE_CONTENTS);

        let result = read_file_to_string("test_data/worm.rle").unwrap();
        assert_eq!(result, WORM_FILE_CONTENTS);
    }

    #[test]
    fn test_create_rleparser() {
        let parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        assert_eq!(parser.get_file_contents(), GLIDER_FILE_CONTENTS);
    }

    #[test]
    fn test_rleparser_get_header() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let header = "x = 3, y = 3, rule = B3/S23".to_string();
        assert_eq!(parser.get_header(), header);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let header = "x = 6, y = 4, rule = B3/S23".to_string();
        assert_eq!(parser.get_header(), header);
    }

    #[test]
    fn test_rleparser_get_name() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let name = "Glider".to_string();
        assert_eq!(parser.get_name(), name);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let name = "Worm".to_string();
        assert_eq!(parser.get_name(), name);
    }

    #[test]
    fn test_rleparser_get_creator() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let creator = "Richard K. Guy".to_string();
        assert_eq!(parser.get_creator(), creator);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let creator = "?".to_string();
        assert_eq!(parser.get_creator(), creator);
    }

    #[test]
    fn test_rleparser_get_comments() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let comments = "The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.\r\nwww.conwaylife.com/wiki/index.php?title=Glider\r\n".to_string();
        assert_eq!(parser.get_comments(), comments);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let comments = "An 11-bit induction coil.\r\nhttps://conwaylife.com/wiki/Worm\r\n".to_string();
        assert_eq!(parser.get_comments(), comments);
    }

    #[test]
    fn test_rleparser_get_pattern() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let pattern = "bob$2bo$3o!".to_string();
        assert_eq!(parser.get_pattern(), pattern);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let pattern = "b2o$obo$o4bo$b5o!".to_string();
        assert_eq!(parser.get_pattern(), pattern);
    }

    #[test]
    fn test_rleparser_get_width() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let width = 3;
        assert_eq!(parser.get_width(), width);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let width = 6;
        assert_eq!(parser.get_width(), width);
    }

    #[test]
    fn test_rleparser_get_height() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let height = 3;
        assert_eq!(parser.get_height(), height);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let height = 4;
        assert_eq!(parser.get_height(), height);
    }

    #[test]
    fn test_rleparser_get_rule() {
        let mut parser = RLEParser::new(&GLIDER_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let rule = "B3/S23";
        assert_eq!(parser.get_rule(), rule);

        let mut parser = RLEParser::new(&WORM_FILE_CONTENTS).unwrap();
        parser.parse_file();
        let rule = "B3/S23";
        assert_eq!(parser.get_rule(), rule);
    }
}
