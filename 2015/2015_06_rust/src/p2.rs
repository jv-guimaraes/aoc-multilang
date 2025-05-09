use super::cord::Cord;

const SIZE: usize = 1000;

pub fn start_matrix() -> Vec<Vec<u32>> {
    vec![vec![0u32; SIZE]; SIZE]
}

pub fn _print_matrix(matrix: &Vec<Vec<u32>>) {
    for line in matrix {
        for row in line {
            print!("{}  ", *row as u8);
        }
        println!()
    }
    println!()
}

pub fn turn_on(matrix: &mut Vec<Vec<u32>>, start: Cord, end: Cord) {
    for j in start.x..end.x + 1 {
        for i in start.y..end.y + 1 {
            matrix[i][j] += 1;
        }
    }
}

pub fn turn_off(matrix: &mut Vec<Vec<u32>>, start: Cord, end: Cord) {
    for j in start.x..end.x + 1 {
        for i in start.y..end.y + 1 {
            matrix[i][j] = matrix[i][j].saturating_sub(1);
        }
    }
}

pub fn toggle(matrix: &mut Vec<Vec<u32>>, start: Cord, end: Cord) {
    for j in start.x..end.x + 1 {
        for i in start.y..end.y + 1 {
            matrix[i][j] += 2;
        }
    }
}

pub fn total_brightness(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            count += matrix[i][j];
        }
    }
    count
}

pub fn run(matrix: &mut Vec<Vec<u32>>, command: &str) {
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