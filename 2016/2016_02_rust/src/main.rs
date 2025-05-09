mod part1;
mod part2;

fn main() {
    let input = include_str!("..\\input.txt");
    // let input = include_str!("..\\sample.txt");
    println!("Code 1: {}", part1::parse_instructions(input));

    // part 2
    println!("Code 2: {}", part2::parse_instructions(input));
}
