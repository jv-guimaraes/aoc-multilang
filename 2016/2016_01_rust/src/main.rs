use std::{fmt::Display, collections::BTreeSet};

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn turn_right(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn turn_left(self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn calculate_coordinates(commands: &str) -> Coord {
    let mut dir = Dir::North;
    let mut coord = Coord {x: 0, y: 0};

    for command in commands.split(", ") {
        let letter = &command[..1];
        let number = &command[1..command.len()].parse::<i32>().unwrap();
        
        dir = if letter == "R" { dir.turn_right() } else { dir.turn_left() };

        match dir {
            Dir::North => coord.y += number,
            Dir::East => coord.x += number,
            Dir::South => coord.y -= number,
            Dir::West => coord.x -= number,
        }
    }
    
    coord
}

fn visit_twice(commands: &str) -> Coord {
    let mut dir = Dir::North;
    let mut coord = Coord {x: 0, y: 0};
    let mut visited = BTreeSet::new();

    for command in commands.split(", ") {
        let letter = &command[..1];
        let number = command[1..command.len()].parse::<i32>().unwrap();
        
        dir = if letter == "R" { dir.turn_right() } else { dir.turn_left() };

        for _ in 0..number {
            match dir {
                Dir::North => coord.y += 1,
                Dir::East => coord.x += 1,
                Dir::South => coord.y -= 1,
                Dir::West => coord.x -= 1,
            }
    
            if !visited.insert(coord) {
                return coord;
            }
        }

    }
    
    coord
}

fn main() {
    let input = include_str!("..\\input.txt");
    let final_coord = calculate_coordinates(input);
    println!("Final coordinate: {}", final_coord);
    println!("Distance: {}", final_coord.distance());

    // part 2
    let final_coord = visit_twice(input);
    println!("\nVisited twice: {}", final_coord);
    println!("Distance: {}", final_coord.distance());
}
