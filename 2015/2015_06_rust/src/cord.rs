#[derive(Debug)]
pub struct Cord {
    pub x: usize,
    pub y: usize,
}

impl Cord {
    pub fn new(x: usize, y: usize) -> Cord {
        Cord { x, y }
    }
}
