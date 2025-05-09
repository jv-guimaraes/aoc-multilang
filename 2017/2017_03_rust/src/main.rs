mod point;

use std::collections::HashMap;

use point::{Point, point};

const INPUT: u32 = 347991;
type Grid = HashMap<Point, u32>;

fn part1() {
    for (i, j) in (3..).step_by(2).zip(1..) {
        let floor = (i-2) * (i-2) + 1;
        let ceil = i * i;
        let mid = j-1 + floor;
        let step = i - 1;
        // println!("{j} | {i} | {floor}-{ceil} | mid:{},{},{},{}", mid, mid+step, mid+step*2, mid+step*3);

        if (floor..ceil).contains(&INPUT) { // input case
            let mut distance = INPUT.abs_diff(mid);
            for m in &[mid, mid+step, mid+step*2, mid+step*3] {
                // println!("{} - {} = {}",INPUT, m, INPUT.abs_diff(*m));
                if INPUT.abs_diff(*m) < distance {
                    distance = INPUT.abs_diff(*m);
                }
            }
            println!("Part 1: i + distance = {}", j + distance);
            return;
        }
    }
}

fn print_grid(grid: &Grid, size: i32) {
    for i in -size+1..size {
        for j in -size+1..size {
            print!("{:^7} ", grid.get(&point(j, -i)).unwrap());
        }
        println!();
    }
}

fn build_grid(size: i32) -> Grid {
    let mut grid: Grid = HashMap::new();
    for i in -size+1..size {
        for j in -size+1..size {
            grid.insert(point(i, j), 0);
        }
    }
    grid.insert(point(0, 0), 1);
    grid
}

fn sum(grid: &Grid, pos: Point) -> u32 {
    let mut sum = 0;
    sum += grid.get(&pos.add(point(1, 0))).unwrap();
    sum += grid.get(&pos.add(point(-1, 0))).unwrap();
    sum += grid.get(&pos.add(point(0, 1))).unwrap();
    sum += grid.get(&pos.add(point(0, -1))).unwrap();
    sum += grid.get(&pos.add(point(1, 1))).unwrap();
    sum += grid.get(&pos.add(point(-1, -1))).unwrap();
    sum += grid.get(&pos.add(point(-1, 1))).unwrap();
    sum += grid.get(&pos.add(point(1, -1))).unwrap();
    sum
}

fn part2() {
    let size = 6;
    let mut grid = build_grid(size);

    let mut pos = point(0, 0);
    let mut sign = 1;
    for i in 1..9 {
        for _ in 0..i {
            pos.x += sign;
            let num = sum(&grid, pos);
            grid.insert(pos, num);
        }
        for _ in 0..i {
            pos.y += sign;
            let num = sum(&grid, pos);
            grid.insert(pos, num);
        }
        sign *= -1;
    }

    println!("Part 2: \n");
    print_grid(&grid, size);
}

fn main() {
    part1();
    part2();
}
