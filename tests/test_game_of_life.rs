use game_of_life::game_of_life::GameOfLife;
use game_of_life::grid::Grid;

#[cfg(test)]
mod tests {
    use super::*;

    static PATTERN: &str = "bob$2bo$3o!";
    static WIDTH: i8 = 3;
    static HEIGHT: i8 = 3;

    #[test]
    fn create_game_of_life() {
        let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        let game_of_life = GameOfLife::new(grid);
        assert_eq!(game_of_life.get_current_generation(), 0);
    }

    #[test]
    fn get_pattern() {
        let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        let game_of_life = GameOfLife::new(grid);
        assert_eq!(game_of_life.get_pattern(), "bob$2bo$3o!");
    }

    #[test]
    fn next_generation() {
        let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        let mut game_of_life = GameOfLife::new(grid);
        game_of_life.next_generation();
        assert_eq!(game_of_life.get_current_generation(), 1);
        assert_eq!(game_of_life.get_grid(),  "...\n*.*\n.**\n.*.");

        game_of_life.next_generation();
        assert_eq!(game_of_life.get_current_generation(), 2);
        assert_eq!(game_of_life.get_grid(),  "...\n..*\n*.*\n.**");
    }

    #[test]
    fn simulate_game() {
        let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        let mut game_of_life = GameOfLife::new(grid);
        game_of_life.simulate_game(3);
        assert_eq!(game_of_life.get_current_generation(), 3);
        assert_eq!(game_of_life.get_grid(), "....\n.*..\n..**\n.**.");

        let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        let mut game_of_life = GameOfLife::new(grid);
        game_of_life.simulate_game(4);
        assert_eq!(game_of_life.get_current_generation(), 4);
        assert_eq!(game_of_life.get_grid(), "....\n..*.\n...*\n.***");
    }
}
