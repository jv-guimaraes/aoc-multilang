use std::fmt::{Display, self};

const ALLOWED_POSITIONS: [[char; 5]; 5] = [
    ['0', '0', '1', '0', '0'],
    ['0', '2', '3', '4', '0'],
    ['5', '6', '7', '8', '9'],
    ['0', 'A', 'B', 'C', '0'],
    ['0', '0', 'D', '0', '0'],
];

#[derive(Debug, Clone, Copy)]
struct KeyCoord {
    x: i8,
    y: i8,
}

impl Display for KeyCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl KeyCoord {
    fn set_x(&mut self, x: i8) {
        if !(0..=4).contains(&x) { return };
        if ALLOWED_POSITIONS[self.y as usize][x as usize] != '0' {
            self.x = x;
        }
    }

    fn set_y(&mut self, y: i8) {
        if !(0..=4).contains(&y) { return };
        if ALLOWED_POSITIONS[y as usize][self.x as usize] != '0' {
            self.y = y;
        }
    }

    fn key_number(&self) -> char {
        ALLOWED_POSITIONS[self.y as usize][self.x as usize]
    }

    fn default() -> KeyCoord {
        KeyCoord { x: 0, y: 2 }
    }
}

pub fn parse_instructions(input: &str) -> String {
    let mut code = String::new();
    let mut coord = KeyCoord::default();
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => coord.set_y(coord.y - 1),
                'D' => coord.set_y(coord.y + 1),
                'L' => coord.set_x(coord.x - 1),
                'R' => coord.set_x(coord.x + 1),
                _ => (),
            }
        }
        code.push(coord.key_number());
    }
    code
}
