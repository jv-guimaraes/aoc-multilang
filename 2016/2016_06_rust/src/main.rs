use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

fn most_common(lines: &[&[u8]], index: usize) -> char {
    let dict = lines.iter().map(|x| x[index]).counts();
    dict.into_iter().max_by_key(|x| x.1).unwrap().0 as char
}

fn least_common(lines: &[&[u8]], index: usize) -> char {
    let dict = lines.iter().map(|x| x[index]).counts();
    dict.into_iter().min_by_key(|x| x.1).unwrap().0 as char
}

fn part1(lines: &[&[u8]]) {
    print!("Part 1: ");
    for i in 0..lines[0].len() {
        print!("{}", most_common(lines, i));
    }
    println!()
}

fn part2(lines: &[&[u8]]) {
    print!("Part 2: ");
    for i in 0..lines[0].len() {
        print!("{}", least_common(lines, i));
    }
    println!()
}

fn main()  {
    let lines = INPUT.lines().map(|x| x.as_bytes()).collect_vec();
    part1(&lines);
    part2(&lines);
}