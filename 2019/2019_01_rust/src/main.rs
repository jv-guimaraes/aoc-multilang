const INPUT: &str = include_str!("..\\input.txt");

fn calculate_fuel(x: i32, acc: i32) -> i32 {
    let result = x / 3 - 2;
    if result <= 0 {
        acc
    } else {
        calculate_fuel(result, acc + result)
    }
}

fn part1() {
    let mut total = 0;
    for line in INPUT.lines().map(|x| x.parse::<i32>().unwrap()) {
        total += line / 3 - 2;
    }
    println!("Part 1: {}", total);
}

fn part2() {
    let total: i32 = INPUT
        .lines()
        .map(|x| calculate_fuel(x.parse::<i32>().unwrap(), 0))
        .sum();
    println!("Part 2: {total}");
}

fn main() {
    part1();
    part2();
}
