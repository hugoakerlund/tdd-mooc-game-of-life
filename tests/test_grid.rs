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
        let pattern: String = grid.grid_to_pattern();
        assert_eq!(pattern, PATTERN);
    }

    #[test]
    fn test_get_cell_at() {
        let mut grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        grid.pattern_to_grid();
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
        let mut grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        grid.pattern_to_grid();

        grid.set_cell_at(0, 0, '*');
        grid.set_cell_at(0, 1, '.');
        grid.set_cell_at(0, 2, '*');

        assert_eq!(grid.get_cell_at(0, 0), '*');
        assert_eq!(grid.get_cell_at(0, 1), '.');
        assert_eq!(grid.get_cell_at(0, 2), '*');

    }

    #[test]
    fn count_live_neighbours() {
        let mut grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        grid.pattern_to_grid();

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
        let mut grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        grid.pattern_to_grid();

        assert_eq!(grid.will_cell_live(0, 0), false);
        assert_eq!(grid.will_cell_live(0, 1), false);
        assert_eq!(grid.will_cell_live(0, 2), true);

        assert_eq!(grid.will_cell_live(1, 0), true);
        assert_eq!(grid.will_cell_live(1, 1), false);
        assert_eq!(grid.will_cell_live(1, 2), true);

        assert_eq!(grid.will_cell_live(2, 0), false);
        assert_eq!(grid.will_cell_live(2, 1), true);
        assert_eq!(grid.will_cell_live(2, 2), true);
    }

}
