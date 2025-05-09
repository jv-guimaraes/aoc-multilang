use md5::{Md5, Digest};

fn main() {
    println!("Problem 1: {}", problem_1());
    println!("Problem 2: {}", problem_2());
}

fn problem_1() -> i32 {
    let mut i = 0;
    loop {
        let mut text = String::from("ckczppom");
        text.push_str(&i.to_string());
        if starts_with_5_zeroes(&text) {
            return i;
        }
        i += 1;
    }
}

fn problem_2() -> i32 {
    let mut i = 0;
    loop {
        let mut text = String::from("ckczppom");
        text.push_str(&i.to_string());
        if starts_with_6_zeroes(&text) {
            return i;
        }
        i += 1;
    }
}

fn starts_with_5_zeroes(text: &str) -> bool {
    let mut hasher = Md5::new();
    hasher.update(text);
    let result = hasher.finalize();
    let bytes: Vec<&u8> = result.iter().take(3).collect();
    if *bytes[0] != 0 { return false }
    if *bytes[1] != 0 { return false }
    if *bytes[2] > 15 { return false }
    true
}

fn starts_with_6_zeroes(text: &str) -> bool {
    let mut hasher = Md5::new();
    hasher.update(text);
    let result = hasher.finalize();
    let bytes: Vec<&u8> = result.iter().take(3).collect();
    if *bytes[0] != 0 { return false }
    if *bytes[1] != 0 { return false }
    if *bytes[2] != 0 { return false }
    true
}
