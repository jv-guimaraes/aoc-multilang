#[derive(Debug, Clone, Copy)]
struct KeyCoord {
    x: i8,
    y: i8,
}

impl KeyCoord {
    fn set_x(&mut self, x: i8) {
        self.x = x;
        self.x = x.clamp(0, 2);
    }
    
    fn set_y(&mut self, y: i8) {
        self.y = y;
        self.y = y.clamp(0, 2);
    }

    fn key_number(&self) -> char {
        match (self.x, self.y) {
            (0, 0) => '1',
            (1, 0) => '2',
            (2, 0) => '3',
            (0, 1) => '4',
            (1, 1) => '5',
            (2, 1) => '6',
            (0, 2) => '7',
            (1, 2) => '8',
            (2, 2) => '9',
            _ => panic!(),
        }
    }

    fn default() -> KeyCoord {
        KeyCoord {x: 1, y: 1}
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