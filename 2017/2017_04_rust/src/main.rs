use itertools::*;

const INPUT: &str = include_str!("..\\input.txt");

fn is_valid_1(text: &str) -> bool {
    text.split(' ').all_unique()
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    let (mut s1, mut s2) = (Vec::from(s1), Vec::from(s2));
    s1.sort(); s2.sort();
    s1 == s2
}

fn is_valid_2(text: &str) -> bool {
    text.split(' ').combinations(2).all(|x| !is_anagram(x[0], x[1]))
}

fn main() {
    let result = INPUT.lines().filter(|line| is_valid_1(line)).count();
    println!("Part 1: {}", result);

    let result = INPUT.lines().filter(|line| is_valid_2(line)).count();
    println!("Part 2: {}", result);
}
