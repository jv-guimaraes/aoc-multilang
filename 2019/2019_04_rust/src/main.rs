const RANGE: std::ops::RangeInclusive<u32> = 271973..=785961;

fn is_valid(password: u32) -> bool {
    let bytes = password.to_string().into_bytes();

    if bytes.windows(2).any(|x| x[0] > x[1]) {
        return false;
    }

    if !bytes.windows(2).any(|x| x[0] == x[1]) {
        return false;
    }

    true
}

fn is_valid2(password: u32) -> bool {
    let b = password.to_string().into_bytes();

    if b.windows(2).any(|x| x[0] > x[1]) {
        return false;
    }

    if b[0] == b[1] && b[1] != b[2] { return true }
    if b[1] == b[2] && b[1] != b[0] && b[1] != b[3] { return true }
    if b[2] == b[3] && b[2] != b[1] && b[2] != b[4] { return true }
    if b[3] == b[4] && b[3] != b[2] && b[3] != b[5] { return true }
    if b[4] == b[5] && b[4] != b[3] { return true }

    false
}

fn main() {
    println!("Part1: {}", RANGE.filter(|x| is_valid(*x)).count());
    println!("Part2: {}", RANGE.filter(|x| is_valid2(*x)).count());
}
