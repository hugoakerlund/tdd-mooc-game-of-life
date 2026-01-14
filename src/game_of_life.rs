use crate::grid::Grid;

pub struct GameOfLife {
    grid: Grid,
    generations: u8
}

impl GameOfLife {
    pub fn new(grid: Grid) -> Self {
        grid.grid_to_pattern();
        Self {
            grid: grid,
            generations: 0
        }
    }

    pub fn get_generations(&self) -> u8 {
        return self.generations;
    }

    pub fn will_cell_live(&self, row: usize, col: usize) -> bool {
        let live_neighbours = self.grid.count_live_neighbours(row as i8, col as i8);
        let is_alive: bool = self.grid.is_alive(row, col);
        if !is_alive && live_neighbours == 3 {
            return true;
        }
        else if is_alive && live_neighbours < 2  || live_neighbours > 3{
            return false
        }
        true
    }
}
