use crate::grid::Grid;

pub struct App {
    pub grid: Grid,
    pub paused: bool,
    pub generation: usize,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            paused: true,
            generation: 0,
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
}
