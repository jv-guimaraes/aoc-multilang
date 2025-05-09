use super::cord::Cord;

const SIZE: usize = 1000;

pub fn start_matrix() -> [[bool; SIZE]; SIZE] {
    [[false; SIZE]; SIZE]
}

pub fn _print_matrix(matrix: &[[bool; SIZE]; SIZE]) {
    for line in matrix {
        for row in line {
            print!("{}  ", *row as u8);
        }
        println!()
    }
    println!()
}

pub fn turn_on(matrix: &mut [[bool; SIZE]; SIZE], start: Cord, end: Cord) {
    for j in start.x..end.x + 1 {
        for i in start.y..end.y + 1 {
            matrix[i][j] = true;
        }
    }
}

pub fn turn_off(matrix: &mut [[bool; SIZE]; SIZE], start: Cord, end: Cord) {
    for j in start.x..end.x + 1 {
        for i in start.y..end.y + 1 {
            matrix[i][j] = false;
        }
    }
}

pub fn toggle(matrix: &mut [[bool; SIZE]; SIZE], start: Cord, end: Cord) {
    for j in start.x..end.x + 1 {
        for i in start.y..end.y + 1 {
            matrix[i][j] = !matrix[i][j];
        }
    }
}

pub fn light_count(matrix: &[[bool; SIZE]; SIZE]) -> u32 {
    let mut count = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            count += matrix[i][j] as u32;
        }
    }
    count
}

pub fn run(matrix: &mut [[bool; SIZE]; SIZE], command: &str) {
    if command.starts_with("toggle") {
        let split_command: Vec<&str> = command.split_whitespace().collect();
        let start: Vec<usize> = split_command[1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let end: Vec<usize> = split_command[3].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let start = Cord::new(start[0], start[1]);
        let end = Cord::new(end[0], end[1]);
        toggle(matrix, start, end);
    } else {
        let split_command: Vec<&str> = command.split_whitespace().collect();
        let start: Vec<usize> = split_command[2].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let end: Vec<usize> = split_command[4].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let start = Cord::new(start[0], start[1]);
        let end = Cord::new(end[0], end[1]);
        if command.starts_with("turn on") {
            turn_on(matrix, start, end);
        } else {
            turn_off(matrix, start, end);
        }
    }
}