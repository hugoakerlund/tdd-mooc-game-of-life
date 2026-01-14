use game_of_life::game_of_life::GameOfLife;
use game_of_life::grid::Grid;

#[cfg(test)]
mod tests {
    use super::*;

    static PATTERN: &str = "bob$2bo$3o!";
    static WIDTH: u8 = 3;
    static HEIGHT: u8 = 3;

    #[test]
    fn test_create_game_of_life() {
        let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
        let game_of_life = GameOfLife::new(grid);
        assert_eq!(game_of_life.get_generations(), 0);
    }

    // #[test]
    // fn test_will_cell_live() {
    //     let grid = Grid::new(PATTERN, WIDTH, HEIGHT);
    //     let game_of_life = GameOfLife::new(grid);

        // assert_eq!(game_of_life.will_cell_live(0, 0), false);
        // assert_eq!(game_of_life.will_cell_live(0, 1), false);
        // assert_eq!(game_of_life.will_cell_live(0, 2), true);
        //
        // assert_eq!(game_of_life.will_cell_live(1, 0), true);
        // assert_eq!(game_of_life.will_cell_live(1, 1), false);
        // assert_eq!(game_of_life.will_cell_live(1, 2), true);
        //
        // assert_eq!(game_of_life.will_cell_live(2, 0), false);
        // assert_eq!(game_of_life.will_cell_live(2, 1), true);
        // assert_eq!(game_of_life.will_cell_live(2, 2), true);
    // }


}
