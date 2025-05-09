#![allow(unused)]
use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

fn load_code_into_i32(raw_code: &str) -> Vec<i32> {
    raw_code.split(',').map(|x| x.parse().unwrap()).collect()
}

fn execute_code(code: &mut [i32], input: i32) {
    let mut i = 0;
    while i < code.len() {
        let instruction = format!("{:05}", code[i]);
        let opcode: usize = instruction[3..5].parse().unwrap();
        let parameters_modes = instruction.chars().collect_vec();
        let p1 = parameters_modes[2];
        let p2 = parameters_modes[1];
        // println!("{:?}, {:?}", &parameters_modes[0..3], opcode);
        match opcode {
            1 | 2 => {
                let first_num = if p1 == '0' {
                    code[code[i + 1] as usize]
                } else {
                    code[i + 1]
                };
                let second_num = if p2 == '0' {
                    code[code[i + 2] as usize]
                } else {
                    code[i + 2]
                };
                code[code[i + 3] as usize] = match opcode {
                    1 => first_num + second_num,
                    2 => first_num * second_num,
                    _ => panic!("{} is not a 1 or 2", opcode),
                };
                i += 4;
            }
            3 => {
                let address = code[i + 1] as usize;
                code[address] = input;
                i += 2;
            }
            4 => {
                let number = match p1 {
                    '0' => code[code[i + 1] as usize],
                    '1' => code[i + 1],
                    _ => panic!("{} is not a 1 or 2", p1),
                };
                print!("{} ", number);
                i += 2;
            }
            _ => {
                break;
            }
        }
    }
}

fn main() {
    let mut code = load_code_into_i32(INPUT);
    // let mut code = load_code_into_i32("3,0,4,0,99");
    execute_code(&mut code, 1);
    // println!("{:?}", code);
}
