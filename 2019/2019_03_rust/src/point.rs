use std::fmt::Display;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn move_up(&mut self) {
        self.y += 1;
    }

    pub fn move_down(&mut self) {
        self.y += -1;
    }

    pub fn move_left(&mut self) {
        self.x += -1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn p(x: i32, y: i32) -> Point {
    Point {x, y}
}