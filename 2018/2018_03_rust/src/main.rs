use std::fmt::Debug;

#[derive(Clone, Copy)]
enum SquareInch {
    Free,
    Single,
    Overlap,
}

impl Debug for SquareInch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free => write!(f, "F"),
            Self::Single => write!(f, "S"),
            Self::Overlap => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Rect {
    fn new(line: &str) -> Rect {
        let line = line.replace(',', " ").replace(": ", " ").replace('x', " ");
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        Rect {
            x: tokens[2].parse().unwrap(),
            y: tokens[3].parse().unwrap(),
            w: tokens[4].parse().unwrap(),
            h: tokens[5].parse().unwrap(),
        }
    }   
}

type Field = [Vec<SquareInch>];

fn load_rectangles() -> Vec<Rect> {
    let input = include_str!("..\\input.txt");
    input.lines().map(Rect::new).collect()
}

fn paint_field(field: &mut Field, rects: &[Rect]) {
    for rect in rects {
        let (x, y) = (rect.x, rect.y);
        for i in 0..rect.w {
            for j in 0..rect.h {
                let (x, y) = ((x + i) as usize, (y + j) as usize);
                field[y][x] = match field[y][x] {
                    SquareInch::Free => SquareInch::Single,
                    SquareInch::Single => SquareInch::Overlap,
                    SquareInch::Overlap => SquareInch::Overlap,
                }
            }
        }
    }
}

fn count_overlaping_area(field: &Field) -> u32 {
    let mut area = 0;
    for row in field {
        for square_inch in row {
            if let SquareInch::Overlap = square_inch {
                area += 1;
            }
        }
    }
    area
}

fn find_rect_not_overlaping(field: &Field, rects: &[Rect]) -> u32 {
    'outer: for (i, rect) in rects.iter().enumerate() {
        let (x, y) = (rect.x, rect.y);
        for i in 0..rect.w {
            for j in 0..rect.h {
                let (x, y) = ((x + i) as usize, (y + j) as usize);
                if let SquareInch::Overlap = field[y][x] {
                    continue 'outer;
                }
            }
        }
        return i as u32 + 1;
    }
    panic!("Couldn't find non-overlaping rect!");
}

fn main() {
    let rects = load_rectangles();
    let mut field = vec![vec![SquareInch::Free; 1000]; 1000];

    paint_field(&mut field, &rects);

    println!("Part 1: {}", count_overlaping_area(&field));
    println!("Part 2: {}", find_rect_not_overlaping(&field, &rects));
}
