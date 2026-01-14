use crate::grid::Grid;

pub struct GameOfLife {
    grid: Grid,
    generations: u8
}

impl GameOfLife {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid: grid,
            generations: 0
        }
    }

    pub fn get_generations(&self) -> u8 {
        return self.generations;
    }
}
