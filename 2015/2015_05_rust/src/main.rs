use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Problem 1: {}", problem_1(&input));
    println!("Problem 2: {}", problem_2(&input));
}

fn problem_1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| acc + is_nice1(line) as i32)
}

fn problem_2(input: &str) -> i32 {
    let mut buffer = String::new();
    let count = input.lines().fold(0, |acc, line| {
        if is_nice2(line) { buffer.push_str(line); buffer.push('\n') };
        acc + is_nice2(line) as i32}
    );
    fs::write("output.txt", buffer).unwrap();
    count
}

fn is_nice2(input: &str) -> bool {
    fn repeat_between(text: &str) -> bool {
        let vec: Vec<char> = text.chars().collect();
        for i in 1..vec.len()-1 {
            if vec[i-1] == vec[i+1] { return true }
        }
        false
    }
    
    fn pair_without_overlapping(text: &str) -> bool {
        let mut map: HashMap<&str, i32> = HashMap::new();
        let mut prev_slice = "-1";
    
        for i in 0..text.len()-1 {
            let slice = text.get(i..i+2).unwrap();
            if slice != prev_slice {
                map.entry(slice).and_modify(|v| *v += 1).or_insert(1);
                prev_slice = slice;
            } else {
                prev_slice = "-1";
            }
            if *map.get(slice).unwrap() >= 2 { return true }
        }
        false
    }

    if !repeat_between(input) { return false }
    if !pair_without_overlapping(input) { return false }
    true
}

fn is_nice1(text: &str) -> bool {
    fn has_three_vowels(text: &str) -> bool {
        let mut vowel_count = 0;
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        for c in text.chars() {
            if vowels.contains(&c) {
                vowel_count += 1;
                if vowel_count >= 3 { return true; }
            }
        }
        false
    }
    
    fn has_twice_in_a_row(text: &str) -> bool {
        let vec: Vec<char> = text.chars().collect();
        for i in 1..vec.len() {
            if vec[i] == vec[i-1] { return true }
        }
        false
    }
    
    fn does_not_contain(text: &str) -> bool {
        let prohibitied = ["ab", "cd", "pq", "xy"];
        for s in prohibitied {
            if text.contains(s) {
                return false
            }
        }
        true
    }

    if !has_three_vowels(text) { return false }
    if !has_twice_in_a_row(text) { return  false }
    if !does_not_contain(text) { return  false }
    true
}
