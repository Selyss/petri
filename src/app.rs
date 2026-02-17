use crate::grid::Grid;

pub struct App {
    pub grid: Grid,
    pub paused: bool,
    pub generation: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            paused: true,
            generation: 0,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn step(&mut self) {
        self.grid.step();
        self.generation += 1;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn randomize(&mut self) {
        self.grid.randomize();
        self.generation = 0;
    }

    pub fn clear(&mut self) {
        self.grid.clear();
        self.generation = 0;
    }

    pub fn toggle_cell(&mut self) {
        let idx = self.cursor_y * self.grid.width + self.cursor_x;
        self.grid.cells[idx] = !self.grid.cells[idx];
    }
}
