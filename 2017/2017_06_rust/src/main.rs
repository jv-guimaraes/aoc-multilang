use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

type Banks = Vec<usize>;

fn load_banks() -> Banks {
    INPUT.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect_vec()
}

fn biggest_bank(banks: &Banks) -> usize {
   let mut biggest_bank_index = 0;
   for (index, bank) in banks.iter().enumerate() {
    if *bank > banks[biggest_bank_index] {
        biggest_bank_index = index;
    }
   }
   biggest_bank_index
}

fn run_reallocation(banks: &mut Banks) {
    let mut index = biggest_bank(banks);
    let mut reallocated_amount = banks[index];
    banks[index] = 0;
    
    while reallocated_amount > 0 {
        index = (index + 1) % banks.len(); // go to the next index in a loop
        banks[index] += 1; // add one block to the current bank
        reallocated_amount -= 1; // remove one from the amount of the biggest bank
    }
}

fn part1() {
    let mut bank_configurations = HashSet::new();
    let mut banks = load_banks();
    let mut reallocation_count = 0;
    
    loop {
        if !bank_configurations.insert(banks.clone()) {
            println!("Part 1: {}", reallocation_count);
            return;
        }
        run_reallocation(&mut banks);
        reallocation_count += 1;
    }
}

fn part2() {
    let mut bank_configurations = HashSet::new();
    let mut banks = load_banks();
    let mut reallocation_count = 0;
    
    loop {
        if !bank_configurations.insert(banks.clone()) {
            break;
        }
        run_reallocation(&mut banks);
        reallocation_count += 1;
    }

    let seen_state = banks.clone();
    reallocation_count = 0;
    loop {
        run_reallocation(&mut banks);
        reallocation_count += 1;
        if banks == seen_state {
            println!("Part 2: {}", reallocation_count);
            return;
        }
    }
}

fn main()  {
    part1();
    part2();
}