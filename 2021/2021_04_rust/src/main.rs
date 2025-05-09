#![allow(unused)]
#![allow(clippy::needless_range_loop)]

use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

type Board = Vec<Vec<Number>>;

enum Number {
    Marked(u32),
    Unmarked(u32),
}

impl Number {
    fn mark(&mut self, num: u32) {
        if let Number::Unmarked(inner_num) = self {
            if num == *inner_num {
                *self = Number::Marked(*inner_num);
            }
        }
    }

    fn is_marked(&self) -> bool {
        match self {
            Number::Marked(_) => true,
            Number::Unmarked(_) => false,
        }
    }

    fn number(&self) -> u32 {
        match self {
            Number::Marked(num) => *num,
            Number::Unmarked(num) => *num,
        }
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Marked(arg0) => write!(f, "|({:2})", arg0),
            Self::Unmarked(arg0) => write!(f, "|{:4}", arg0),
        }
    }
}

impl std::str::FromStr for Number {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Number::Unmarked(s.parse::<u32>()?))
    }
}

fn print_board(board: &Board) {
    for row in board {
        for num in row {
            print!("{:?} ", num);
        }
        println!();
    }
    println!();
}

fn str_to_board(board_str: &str) -> Board {
    let mut board: Board = Vec::new();
    for line in board_str.lines() {
        board.push(line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect_vec())
    }
    board
}

fn load_boards() -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    for board_str in INPUT.split("\r\n\r\n").skip(1) {
        boards.push(str_to_board(board_str));
    }
    boards
}

fn bingo(board: &Board) -> bool {
    for row in board {
        if row.iter().all(|x| x.is_marked()) {
            return true;
        }
    }

    let mut marked_count = 0;
    for col in 0..5 {
        for row in 0..5 {
            if board[row][col].is_marked() {
                marked_count += 1;
            }
        }
        if marked_count == 5 { return true }
        marked_count = 0;
    }
    
    false
}

fn unmarked_sum(board: &Board) -> u32 {
    let mut sum = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !board[row][col].is_marked() {
                sum += board[row][col].number();
            }
        }
    }
    sum
}

fn part1() {
    let rng: Vec<u32> = INPUT.lines().next().unwrap().split(',').map(|x|x.parse().unwrap()).collect();
    let mut boards = load_boards();

    for rand_num in rng {
        // Mark each board
        for board in boards.iter_mut() {
            for row in board {
                for number in row {
                    number.mark(rand_num);
                }
            }
        }
        // Check if any board got bingo
        for board in boards.iter() {
            if bingo(board) {
                // print_board(board);
                println!("Part 1: {}", rand_num * unmarked_sum(board));
                return;
            }
        }
    }
}

fn main() {
    part1()
}