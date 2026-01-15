use::game_of_life::parse_cmd_arguments;
use game_of_life::read_file_to_string;
use game_of_life::rle_parser::RLEParser;
use game_of_life::grid::Grid;
use game_of_life::game_of_life::GameOfLife;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_arguments: (u8, String) = parse_cmd_arguments(args).unwrap();
    let generations = parsed_arguments.0;
    let file = parsed_arguments.1;
    let file_contents = read_file_to_string(&file).unwrap();

    let mut rle_parser = RLEParser::new(&file_contents).unwrap();
    rle_parser.parse_file();
    let pattern = rle_parser.get_pattern();
    let width = rle_parser.get_width();
    let height = rle_parser.get_height();

    let grid = Grid::new(&pattern, width, height);
    let mut game_of_life = GameOfLife::new(grid);
    game_of_life.simulate_game(generations);
    let pattern = game_of_life.get_pattern();
    println!("{}", pattern);
}
