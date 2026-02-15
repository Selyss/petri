pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
    scratch: Vec<bool>, // scratch buffer
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![false; size],
            scratch: vec![false; size],
        }
    }

    pub fn step(&mut self) {
        // TODO: implement rules, write result to scratch buffer, swap buffers
        std::mem::swap(&mut self.cells, &mut self.scratch);
    }

    pub fn randomize(&mut self) {
        // TODO: fill with random (or allow for custom positions)
        todo!()
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
    }
}
