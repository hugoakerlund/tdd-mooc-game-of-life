use crate::grid::Grid;

pub struct GameOfLife {
    grid: Grid,
    generations: u8
}

impl GameOfLife {
    pub fn new(grid: Grid) -> Self {
        let mut game_grid = grid;
        game_grid.pattern_to_grid();
        Self {
            grid: game_grid,
            generations: 0
        }
    }

    pub fn get_current_generation(&self) -> u8 {
        return self.generations;
    }

    pub fn get_grid(&self) -> String {
        self.grid.to_string()
    }

    pub fn get_pattern(&self) -> String {
        self.grid.grid_to_pattern()
    }

    pub fn simulate_game(&mut self, generations: u8) {
        for _i in 0..generations {
            self.next_generation();
        }
    }

    pub fn next_generation(&mut self) {
        self.generations += 1;
        self.grid.next_generation();
    }

}
