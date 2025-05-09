const INPUT: &str = include_str!("..\\input.txt");

fn binary_to_u32(binary: &str) -> u32 {
    u32::from_str_radix(binary, 2).unwrap()
}

fn most_common_bit(index: usize, lines: &[&str]) -> char {
    let mut count = 0;
    for line in lines.iter().map(|l| l.as_bytes()) {
        if line[index] == b'1' {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count < 0 {
        '0'
    } else {
        '1'
    }
}

fn least_common_bit(index: usize, lines: &[&str]) -> char {
    let mut count = 0;
    for line in lines.iter().map(|l| l.as_bytes()) {
        if line[index] == b'1' {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count < 0 {
        '1'
    } else {
        '0'
    }
}

fn part1() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let mut gamma = String::new();
    for i in 0..lines[0].len() {
        gamma.push(most_common_bit(i, &lines));
    }
    let epsilon: String = gamma.bytes().map(|b| if b == b'1' {'0'} else {'1'}).collect();
    let epsilon = binary_to_u32(&epsilon);
    let gamma = binary_to_u32(&gamma);
    println!("Part 1: {}", epsilon * gamma);
}

fn part2() {
    //oxygen generator rating
    let mut lines: Vec<&str> = INPUT.lines().collect();
    for i in 0..lines[0].len() {
        let bit = most_common_bit(i, &lines) as u8;
        lines.retain(|x| x.as_bytes()[i] == bit);
        if lines.len() <= 1 {
            break;
        }
    }
    let oxygen = binary_to_u32(lines[0]);

    // CO2 scrubber rating
    let mut lines: Vec<&str> = INPUT.lines().collect();
    for i in 0..lines[0].len() {
        let bit = least_common_bit(i, &lines) as u8;
        lines.retain(|x| x.as_bytes()[i] == bit);
        if lines.len() <= 1 {
            break;
        }
    }
    let co2 = binary_to_u32(lines[0]);
    println!("Part 2: {}", oxygen * co2);

}

fn main()  {
    part1();
    part2();
}