use game_of_life::grid::Grid;

#[cfg(test)]
mod tests {
    use super::*;

    static PATTERN: &str = "bob$2bo$3o!";
    static WIDTH: u8 = 3;
    static HEIGHT: u8 = 3;

    #[test]
    fn test_create_grid() {
        let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        assert_eq!(grid.get_width(), 3);
        assert_eq!(grid.get_height(), 3);
        assert_eq!(grid.get_pattern(), PATTERN);
    }

    #[test]
    fn test_pattern_to_grid() {
        let mut grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        grid.pattern_to_grid();
        let expected = ".*.\n..*\n***".to_string();
        assert_eq!(grid.to_string(), expected);
    }

    #[test]
    fn test_grid_to_pattern() {
        let mut grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        grid.pattern_to_grid();
        let pattern = grid.grid_to_pattern();
        assert_eq!(pattern, PATTERN);
    }

}

