const INPUT: &str = include_str!("..\\input.txt");

fn part1() {
    let mut jumps: Vec<i32> = INPUT.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let len = jumps.len() as i32;
    
    let mut steps = 0;
    let mut i: i32 = 0;
    loop {
        steps += 1;
        
        let next_i = i + jumps[i as usize];
        if next_i >= len { break; }
        
        jumps[i as usize] += 1;
        i = next_i;
    }
    println!("Part 1: {}", steps);
}

fn part2() {
    let mut jumps: Vec<i32> = INPUT.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let len = jumps.len() as i32;
    
    let mut steps = 0;
    let mut i: i32 = 0;
    loop {
        steps += 1;
        
        let next_i = i + jumps[i as usize];
        if next_i >= len { break; }
        
        if jumps[i as usize] >= 3 {
            jumps[i as usize] -= 1;
        } else {
            jumps[i as usize] += 1;
        }
        i = next_i;
    }
    println!("Part 1: {}", steps);
}

fn main()  {
    part1();
    part2();
}