use std::collections::HashSet;

const INPUT: &str = include_str!("..\\input.txt");

fn find_overlap(bag: &str) -> char {
    let compart_size = bag.len()/2;
    let first_compartment_set: HashSet<u8> = HashSet::from_iter(bag.bytes().take(compart_size));
    for item in bag.bytes().skip(compart_size) {
        if first_compartment_set.contains(&item) {
            return item as char;
        }
    }
    panic!("Couldn't find overlapping item!");
}

fn char_priority(c: char) -> i32 {
    if c as i32 >= 97 {
        c as i32 - 96
    } else {
        c as i32 - 38
    }
}

fn part1() {
    let mut sum = 0;
    for bag in INPUT.lines() {
        let item = find_overlap(bag);
        sum += char_priority(item);
    }
    println!("Part 1: {}", sum);
}

fn find_group_overlap(group: &[&str]) -> char {
    let e1: HashSet<char> = group[0].chars().collect();
    let e2: HashSet<char> = group[1].chars().collect();
    let e3: HashSet<char> = group[2].chars().collect();
    *e1.intersection(&e2).find(|x| e3.contains(x)).expect("Failed to find group overlap")
}

fn part2() {
    let mut sum = 0;
    for group in INPUT.lines().collect::<Vec<&str>>().chunks(3) {
        let item = find_group_overlap(group);
        sum += char_priority(item);
    }
    println!("Part 2: {}", sum);
}

fn main()  {
    part1();
    part2();
}