const INPUT: &str = include_str!("..\\input.txt");

fn load_code(noun: usize, verb: usize) -> Vec<usize> {
    assert!((0..=99).contains(&noun) && (0..=99).contains(&verb));
    let mut code: Vec<usize> = INPUT
        .trim_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    (code[1], code[2]) = (noun, verb);
    code
}

fn execute_code(code: &mut [usize]) {
    for i in (0..code.len()).step_by(4) {
        let opcode = code[i];
        if opcode == 99 { // END
            return;
        }
        let (arg1, arg2, arg3) = (code[i+1], code[i+2], code[i+3]);
        if opcode == 1 { // ADDITION
            code[arg3] = code[arg1] + code[arg2];
        }
        else if opcode == 2 { // MULTIPLICATION
            code[arg3] = code[arg1] * code[arg2];
        } else {
            panic!("Incorrect opcode: {opcode}");
        }
    }
}

fn run_with_input(noun: usize, verb: usize) -> usize {
    let mut code = load_code(noun, verb);
    execute_code(&mut code);
    code[0]
}

fn part1() {
    let mut code = load_code(12, 2);
    execute_code(&mut code);
    println!("Part 1: {:?}", code[0]);
}

fn part2() {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_with_input(noun, verb) == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
