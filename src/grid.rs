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

    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0u8;
        for dy in [-1isize, 0, 1] {
            for dx in [-1isize, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                if self.cells[ny * self.width + nx] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn step(&mut self) {
        // TODO: implement rules, write result to scratch buffer, swap buffers
        for x in 0..self.height {
            for y in 0..self.width {
                let idx = y * self.width + x;
                let neighbors = self.count_neighbors(x, y);
                let alive = self.cells[idx];

                self.scratch[idx] =
                    matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
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
