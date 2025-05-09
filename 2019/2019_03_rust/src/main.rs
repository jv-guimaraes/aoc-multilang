use std::collections::HashSet;

use point::*;

mod point;

const INPUT: &str = include_str!("..\\input.txt");

fn calculate_wire_positions(wire: &mut Vec<Point>, input: &str) {
    let mut pos = p(0, 0);
    for command in input.split(',') {
        let move_len: i32 = command[1..].parse().unwrap();
        if command.starts_with('U') {
            for _ in 0..move_len {
                pos.move_up();
                wire.push(pos);
            }
        } else if command.starts_with('D') {
            for _ in 0..move_len {
                pos.move_down();
                wire.push(pos);
            }
        } else if command.starts_with('L') {
            for _ in 0..move_len {
                pos.move_left();
                wire.push(pos);
            }
        } else {
            for _ in 0..move_len {
                pos.move_right();
                wire.push(pos);
            }
        }
    }
}

fn part1(intersections: &[Point]) {
    let closest = intersections.iter().min_by(|x, y| x.distance().cmp(&y.distance())).unwrap();
    println!("Part 1: {}", closest.distance());
}

fn part2(wire1: &[Point], wire2: &[Point], intersections: &[Point]) {
    let mut closest_distance = i32::MAX;
    for intersection in intersections {
        let mut moves = 0;
        for wire in [wire1, wire2] {
            for (i, point) in wire.iter().enumerate() {
                if point == intersection {
                    moves += i as i32 + 1;
                    break;
                }
            }
        }
        if moves < closest_distance {
            closest_distance = moves;
        }
    }
    println!("Part 2: {}", closest_distance);
}

fn main() {
    let input: Vec<&str> = INPUT.lines().collect();
    let mut wire1: Vec<Point> = Vec::new();
    let mut wire2: Vec<Point> = Vec::new();
    calculate_wire_positions(&mut wire1, input[0]);
    calculate_wire_positions(&mut wire2, input[1]);
    let wire1_set: HashSet<&Point> = HashSet::from_iter(wire1.iter());
    let wire2_set: HashSet<&Point> = HashSet::from_iter(wire2.iter());
    let intersections: Vec<Point> = wire1_set.intersection(&wire2_set).map(|x|**x).collect();
    part1(&intersections);
    part2(&wire1, &wire2, &intersections);
}
