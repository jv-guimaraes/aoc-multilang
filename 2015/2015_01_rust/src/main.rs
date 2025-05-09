use std::fs;

fn read_line(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn what_floor(moves: &str) -> i32 {
    let ups = moves.chars().filter(|x| *x == '(').count() as i32;
    let downs = moves.chars().filter(|x| *x == ')').count() as i32;
    ups - downs
}

fn when_enter_basement(moves: &str) -> usize {
    let mut current_floor = 0;
    for (i, c) in moves.chars().enumerate() {
        if c == '(' {
            current_floor += 1;
        } else {
            current_floor -= 1;
        }

        if current_floor < 0 {
            return i + 1;
        }
    }
    0
}

fn main() {
    let line = read_line("input.txt");
    println!("{}\n", what_floor(&line));
    println!("{}", when_enter_basement(&line));
}
