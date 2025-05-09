const INPUT: &str = include_str!("..\\input.txt");

fn is_valid_1(password: &str) -> bool {
    let tokens: Vec<&str> = password.split(' ').collect();
    let mut floor_ceil = tokens[0].split('-').map(|x| x.parse::<usize>().unwrap());
    let floor = floor_ceil.next().unwrap();
    let ceil = floor_ceil.next().unwrap();
    let letter = tokens[1].chars().next().unwrap();
    let match_count = tokens[2].matches(letter).count();
    match_count >= floor && match_count <= ceil
}

fn is_valid_2(password: &str) -> bool {
    let tokens: Vec<&str> = password.split(' ').collect();
    let mut floor_ceil = tokens[0].split('-').map(|x| x.parse::<usize>().unwrap());
    let i = floor_ceil.next().unwrap() - 1;
    let j = floor_ceil.next().unwrap() - 1;
    let letter = tokens[1].bytes().next().unwrap();
    let text = tokens[2].as_bytes();
    letter == text[i]  && letter != text[j] ||
    letter == text[j]  && letter != text[i] 
}

fn main() {
    println!("Part 1 {:?}", INPUT.lines().filter(|x| is_valid_1(x)).count());
    println!("Part 2 {:?}", INPUT.lines().filter(|x| is_valid_2(x)).count());
}
