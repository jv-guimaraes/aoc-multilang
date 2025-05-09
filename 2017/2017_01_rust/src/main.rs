fn main() {
    let input = include_str!("..\\input.txt");
    let digits = input.bytes().map(|b| (b as char).to_digit(10).unwrap()).collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..digits.len()-1 {
        if digits[i] == digits[i+1] {
            sum += digits[i];
        }
    }
    if digits[0] == *digits.last().unwrap() { sum += digits[0] };
    println!("Part 1: {sum}");

    // part 2
    let len = digits.len();
    let half = digits.len() / 2;
    let mut sum = 0;
    for i in 0..len-1 {
        if digits[i] == digits[(i + half) % len] {
            sum += digits[i];
        }
    }
    println!("Part 2: {sum}");
}
