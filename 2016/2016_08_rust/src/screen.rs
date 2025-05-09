use std::fmt::Debug;
use itertools::Itertools;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

#[derive(Debug, Clone, Copy)]
enum Pixel {
    On,
    Off,
}

impl Pixel {
    fn char(self) -> char {
        match self {
            Pixel::On => '#',
            Pixel::Off => '.',
        }
    }

    fn is_on(self) -> bool {
        match self {
            Pixel::On => true,
            Pixel::Off => false,
        }
    }
}

pub struct Screen {
    grid: [[Pixel; WIDTH]; HEIGHT],
}

impl Debug for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for row in self.grid {
            for pixel in row {
                buffer.push(pixel.char())
            }
            buffer.push('\n');
        }
        
        write!(f, "{}", buffer)
    }
}

impl Screen {
    pub fn new() -> Screen {
        Screen { grid: [[Pixel::Off; WIDTH]; HEIGHT] }
    }

    pub fn draw_rect(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for col in 0..width {
                self.grid[row][col] = Pixel::On;
            }
        }
    }

    pub fn rotate_col(&mut self, col: usize, by: usize) {
        let mut new_col = [Pixel::Off; HEIGHT];
        for row in 0..HEIGHT {
            let new_row_index = (row + by) % HEIGHT;
            new_col[new_row_index] = self.grid[row][col];
        }
        #[allow(clippy::needless_range_loop)]
        for row in 0..HEIGHT {
            self.grid[row][col] = new_col[row];
        }
    }

    pub fn rotate_row(&mut self, row: usize, by: usize) {
        let mut new_row = [Pixel::Off; WIDTH];
        for col in 0..WIDTH {
            let new_col_index = (col + by) % WIDTH;
            new_row[new_col_index] = self.grid[row][col];
        }
        self.grid[row] = new_row;
    }

    pub fn execute(&mut self, command: &str) {
        // rect 1x1 | rotate row y=0 by 5 | rotate column x=1 by 4
        let tokens = command.split(' ').collect_vec();
        if tokens[0] == "rect" {
            let (w, h) = tokens[1].split('x').collect_tuple().unwrap();
            let (w, h) = (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap());
            self.draw_rect(w, h);
        } else if tokens[1] == "row" {
            let row: usize = tokens[2].split('=').nth(1).unwrap().parse().unwrap();
            let by: usize = tokens[4].parse().unwrap();
            self.rotate_row(row, by);
        } else if tokens[1] == "column" {
            let col: usize = tokens[2].split('=').nth(1).unwrap().parse().unwrap();
            let by: usize = tokens[4].parse().unwrap();
            self.rotate_col(col, by);
        }
    }

    pub fn on_pixels(&self) -> u32 {
        let mut count = 0;
        for row in self.grid {
            for pixel in row {
                if pixel.is_on() {
                    count += 1;
                }
            }
        }
        count
    }
}