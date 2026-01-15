use game_of_life::parse_cmd_arguments;
use game_of_life::print_usage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmd_arguments() {
        let args: Vec<String> = vec!["target/debug/game-of-life".to_string(), "2".to_string(), "name_of_file".to_string()];
        let result: (u8, String) = parse_cmd_arguments(args).unwrap();
        assert_eq!(result.0, 2);
        assert_eq!(result.1, "name_of_file");
    }

    #[test]
    fn test_print_usage() {
        let output: String = print_usage();
        let expected = "Usage: ./game_of_life <generations> <input_file.rle>";
        assert_eq!(output, expected);
    }
}
