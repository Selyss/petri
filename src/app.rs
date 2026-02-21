use std::time::Duration;

use crate::grid::Grid;

pub struct App {
    pub grid: Grid,
    pub paused: bool,
    pub generation: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub tick_rate: Duration,
    pub cursor_visible: bool,
    pub pattern_mode: bool,
    pub view_width: usize,
    pub view_height: usize,
    pub viewport_x: usize,
    pub viewport_y: usize,
    pub zoom: i32,
}

impl App {
    pub fn new(width: usize, height: usize, view_width: usize, view_height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            paused: true,
            generation: 0,
            cursor_x: 0,
            cursor_y: 0,
            tick_rate: Duration::from_millis(100),
            cursor_visible: false,
            pattern_mode: false,
            view_width,
            view_height,
            viewport_x: 0,
            viewport_y: 0,
            zoom: 1,
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
        self.grid.cells[idx] = if self.grid.cells[idx] > 0 { 0 } else { 1 };
    }

    pub fn move_left(&mut self) {
        self.cursor_x = (self.cursor_x as isize - 1).rem_euclid(self.grid.width as isize) as usize;
    }

    pub fn move_down(&mut self) {
        self.cursor_y = (self.cursor_y as isize + 1).rem_euclid(self.grid.height as isize) as usize;
    }

    pub fn move_up(&mut self) {
        self.cursor_y = (self.cursor_y as isize - 1).rem_euclid(self.grid.height as isize) as usize;
    }

    pub fn move_right(&mut self) {
        self.cursor_x = (self.cursor_x as isize + 1).rem_euclid(self.grid.width as isize) as usize;
    }

    pub fn speed_up(&mut self) {
        self.tick_rate = self
            .tick_rate
            .saturating_sub(Duration::from_millis(25))
            .max(Duration::from_millis(10));
    }

    pub fn slow_down(&mut self) {
        self.tick_rate =
            (self.tick_rate + Duration::from_millis(25)).min(Duration::from_millis(1000));
    }

    pub fn toggle_cursor(&mut self) {
        self.cursor_visible = !self.cursor_visible;
    }

    pub fn zoom_in(&mut self) {
        if self.zoom < 8 {
            self.zoom += 1;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom > 0 {
            self.zoom -= 1;
        }
    }

    pub fn place_pattern(&mut self, patterns: &crate::patterns::Pattern) {
        for &(dx, dy) in patterns.cells {
            let x = (self.cursor_x as isize + dx).rem_euclid(self.grid.width as isize) as usize;
            let y = (self.cursor_y as isize + dy).rem_euclid(self.grid.height as isize) as usize;
            let idx = y * self.grid.width + x;
            self.grid.cells[idx] = 1;
        }
    }
}
