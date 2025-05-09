use std::{fs, collections::HashMap};

fn read_line(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn problem_1(input: &str) -> i32 {
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    map.insert(pos, true);
    for c in input.trim().chars() {
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -=1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => ()
        }
        map.insert(pos, true);
    }
    map.len() as i32
}

fn problem_2(input: &str) -> i32 {
    let mut map = HashMap::new();
    
    // Santa
    let mut pos = (0, 0);
    map.insert(pos, true);
    for c in input.trim().chars().step_by(2) {
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -=1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => ()
        }
        map.insert(pos, true);
    }

    // Robo-santa
    pos = (0, 0);
    map.insert(pos, true);
    for c in input.trim().chars().skip(1).step_by(2) {
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -=1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => ()
        }
        map.insert(pos, true);
    }

    map.len() as i32
}

fn main() {
    let input = read_line("input.txt");
    println!("Problem 1: {}", problem_1(&input));
    println!("Problem 2: {}", problem_2(&input));
}
