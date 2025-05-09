mod range;

use itertools::Itertools;
use range::Range;

const INPUT: &str = include_str!("..\\input.txt");

fn load_pairs() -> Vec<(Range, Range)> {
    let mut pairs = Vec::new();
    for pair in INPUT.lines() {
        let (r1, r2) = pair.split(',').map(Range::from_str).collect_tuple().unwrap();
        pairs.push((r1, r2));
    }
    pairs
}

fn main()  {
    let pairs = load_pairs();
    let result = pairs.iter().filter(|(a, b)| a.either_inside(b)).count();
    println!("Part 1: {}", result);

    let result = pairs.iter().filter(|(a, b)| a.overlap(b)).count();
    println!("Part 2: {}", result);
}