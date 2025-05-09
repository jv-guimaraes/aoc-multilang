#![allow(unused)]
use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

fn reduce(polymer: &str) -> String {
    let mut result = String::new();
    let mut polymer = polymer.chars().collect_vec();
    polymer.push('3'); // padding
    let mut i = 0;
    while i < polymer.len() - 1 {
        if polymer[i].to_ascii_lowercase() == polymer[i + 1].to_ascii_lowercase()
            && polymer[i] != polymer[i + 1]
        {
            i += 2;
        } else {
            result.push(polymer[i]);
            i += 1;
        }
    }
    result
}

fn fully_reduce(polymer: &str) -> String {
    let mut polymer = polymer.to_owned();
    loop {
        let reduced = reduce(&polymer);
        if reduced.len() == polymer.len() {
            polymer = reduced;
            break;
        }
        polymer = reduced;
    }
    polymer
}

fn part1(polymer: &str) -> usize {
    let mut polymer = polymer.to_owned();
    fully_reduce(&polymer).len()
}

fn part2(polymer: &str) -> usize {
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut shortest = usize::MAX;
    for c in alphabet.into_iter() {
        let polymer = polymer.replace(c, "");
        let polymer = polymer.replace(c.to_ascii_uppercase(), "");
        shortest = usize::min(shortest, fully_reduce(&polymer).len());
    }
    shortest
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}
