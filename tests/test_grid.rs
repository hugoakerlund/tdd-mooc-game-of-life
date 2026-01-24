use game_of_life::grid::Grid;

#[cfg(test)]
mod tests {
    use super::*;
    
    static GLIDER_PATTERN: &str = "bo$2bo$3o!";
    static GLIDER_WIDTH: i8 = 3;
    static GLIDER_HEIGHT: i8 = 3;

    static WORM_PATTERN: &str = "b2o$obo$o4bo$b5o!";
    static WORM_WIDTH: i8 = 6;
    static WORM_HEIGHT: i8 = 4;

    static EMPTY_LINES_UNCOMPRESSED: &str = "$$$!";
    static EMPTY_WIDTH: i8 = 6;
    static EMPTY_HEIGHT: i8 = 4;

    static EMPTY_LINES_COMPRESSED: &str = "3$!";

    #[test]
    fn test_create_grid() {
        let grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        assert_eq!(grid.get_pattern_width(), GLIDER_WIDTH);
        assert_eq!(grid.get_pattern_height(), GLIDER_HEIGHT);
        assert_eq!(grid.get_pattern(), GLIDER_PATTERN);

        let grid = Grid::new(WORM_PATTERN, WORM_WIDTH, WORM_HEIGHT);
        assert_eq!(grid.get_pattern_width(), WORM_WIDTH);
        assert_eq!(grid.get_pattern_height(), WORM_HEIGHT);
        assert_eq!(grid.get_pattern(), WORM_PATTERN);
    }

    #[test]
    fn test_pattern_to_grid() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();
        let expected = ".*.\n..*\n***".to_string();
        assert_eq!(grid.to_string(), expected);

        let mut grid = Grid::new(WORM_PATTERN, WORM_WIDTH, WORM_HEIGHT);
        grid.rle_pattern_to_grid();
        let expected = ".**...\n*.*...\n*....*\n.*****".to_string();
        assert_eq!(grid.to_string(), expected);

        let mut grid = Grid::new(EMPTY_LINES_UNCOMPRESSED, EMPTY_WIDTH, EMPTY_HEIGHT);
        grid.rle_pattern_to_grid();
        let expected = "......\n......\n......\n......".to_string();
        assert_eq!(grid.to_string(), expected);
    }

    #[test]
    fn test_grid_to_pattern() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();
        let pattern: String = grid.grid_to_rle_pattern();
        assert_eq!(pattern, GLIDER_PATTERN);

        let mut grid = Grid::new(WORM_PATTERN, WORM_WIDTH, WORM_HEIGHT);
        grid.rle_pattern_to_grid();
        let pattern: String = grid.grid_to_rle_pattern();
        assert_eq!(pattern, WORM_PATTERN);

        let mut grid = Grid::new(EMPTY_LINES_UNCOMPRESSED, EMPTY_WIDTH, EMPTY_HEIGHT);
        grid.rle_pattern_to_grid();
        let pattern: String = grid.grid_to_rle_pattern();
        assert_eq!(pattern, EMPTY_LINES_COMPRESSED);

        let mut grid = Grid::new(EMPTY_LINES_COMPRESSED, EMPTY_WIDTH, EMPTY_HEIGHT);
        grid.rle_pattern_to_grid();
        let pattern: String = grid.grid_to_rle_pattern();
        assert_eq!(pattern, EMPTY_LINES_COMPRESSED);
    }

    #[test]
    fn test_get_cell_at() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();
        assert_eq!(grid.get_cell_at(0, 0), '.');
        assert_eq!(grid.get_cell_at(0, 1), '*');
        assert_eq!(grid.get_cell_at(0, 2), '.');

        assert_eq!(grid.get_cell_at(1, 0), '.');
        assert_eq!(grid.get_cell_at(1, 1), '.');
        assert_eq!(grid.get_cell_at(1, 2), '*');

        assert_eq!(grid.get_cell_at(2, 0), '*');
        assert_eq!(grid.get_cell_at(2, 1), '*');
        assert_eq!(grid.get_cell_at(2, 2), '*');
    }

    #[test]
    fn test_set_cell_at() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();

        grid.set_cell_at(0, 0, '*');
        grid.set_cell_at(0, 1, '.');
        grid.set_cell_at(0, 2, '*');

        assert_eq!(grid.get_cell_at(0, 0), '*');
        assert_eq!(grid.get_cell_at(0, 1), '.');
        assert_eq!(grid.get_cell_at(0, 2), '*');

    }

    #[test]
    fn test_count_live_neighbours() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();

        assert_eq!(grid.count_live_neighbours(0, 0), 1);
        assert_eq!(grid.count_live_neighbours(0, 1), 1);
        assert_eq!(grid.count_live_neighbours(0, 2), 2);

        assert_eq!(grid.count_live_neighbours(1, 0), 3);
        assert_eq!(grid.count_live_neighbours(1, 1), 5);
        assert_eq!(grid.count_live_neighbours(1, 2), 3);

        assert_eq!(grid.count_live_neighbours(2, 0), 1);
        assert_eq!(grid.count_live_neighbours(2, 1), 3);
        assert_eq!(grid.count_live_neighbours(2, 2), 2);
    }

    #[test]
    fn test_will_cell_live() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();

        assert_eq!(grid.will_cell_live(0, 0), false);
        assert_eq!(grid.will_cell_live(0, 1), false);
        assert_eq!(grid.will_cell_live(0, 2), false);

        assert_eq!(grid.will_cell_live(1, 0), true);
        assert_eq!(grid.will_cell_live(1, 1), false);
        assert_eq!(grid.will_cell_live(1, 2), true);

        assert_eq!(grid.will_cell_live(2, 0), false);
        assert_eq!(grid.will_cell_live(2, 1), true);
        assert_eq!(grid.will_cell_live(2, 2), true);

        assert_eq!(grid.will_cell_live(3, 0), false);
        assert_eq!(grid.will_cell_live(3, 1), true);
        assert_eq!(grid.will_cell_live(3, 2), false);

    }

    #[test]
    fn test_detect_if_grid_needs_expansion() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();
        let result = grid.detect_if_needs_expansion();
        assert_eq!(result, (true, 0, 1, 0, 0));
    }

    #[test]
    fn test_expand_grid() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();
        grid.expand_grid(1, 1, 1, 1);
        let expected = ".....\n..*..\n...*.\n.***.\n.....";
        assert_eq!(grid.to_string(), expected);
    }

    #[test]
    fn test_next_generation() {
        let mut grid = Grid::new(GLIDER_PATTERN, GLIDER_WIDTH, GLIDER_HEIGHT);
        grid.rle_pattern_to_grid();
        grid.next_generation();
        let expected = "...\n*.*\n.**\n.*.";

        assert_eq!(grid.to_string(), expected);
        grid.next_generation();
        assert_eq!(grid.to_string(), "...\n..*\n*.*\n.**");
    }
}
