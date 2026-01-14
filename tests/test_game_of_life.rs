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


}
