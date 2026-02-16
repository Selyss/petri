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
        for y in 0..self.height {
            for x in 0..self.width {
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
        // TODO: allow for custom start positions
        for y in 0..self.height {
            for x in 0..self.width {
                let alive = fastrand::f64() < 0.25;
                self.cells[y * self.width + x] = alive;
            }
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a grid and set specific cells alive
    fn grid_from_points(width: usize, height: usize, alive: &[(usize, usize)]) -> Grid {
        let mut grid = Grid::new(width, height);
        for &(x, y) in alive {
            grid.cells[y * width + x] = true;
        }
        grid
    }

    /// Helper: collect all alive cell positions
    fn alive_cells(grid: &Grid) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.cells[y * grid.width + x] {
                    result.push((x, y));
                }
            }
        }
        result.sort();
        result
    }

    #[test]
    fn block_still_life() {
        // 2x2 block should never change
        let mut grid = grid_from_points(6, 6, &[(1, 1), (2, 1), (1, 2), (2, 2)]);
        for _ in 0..5 {
            grid.step();
            assert_eq!(alive_cells(&grid), vec![(1, 1), (1, 2), (2, 1), (2, 2)]);
        }
    }

    #[test]
    fn blinker_oscillator() {
        // Vertical line of 3 should toggle to horizontal and back
        let vertical = vec![(2, 1), (2, 2), (2, 3)];
        let horizontal = vec![(1, 2), (2, 2), (3, 2)];

        let mut grid = grid_from_points(6, 6, &vertical);

        grid.step();
        assert_eq!(alive_cells(&grid), horizontal);

        grid.step();
        assert_eq!(alive_cells(&grid), vertical);
    }

    #[test]
    fn empty_grid_stays_empty() {
        let mut grid = Grid::new(10, 10);
        grid.step();
        assert!(alive_cells(&grid).is_empty());
    }

    #[test]
    fn lone_cell_dies() {
        let mut grid = grid_from_points(6, 6, &[(3, 3)]);
        grid.step();
        assert!(alive_cells(&grid).is_empty());
    }
}
