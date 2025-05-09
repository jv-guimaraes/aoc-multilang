const INPUT: &str = "uqwqemis";

fn part1() {
    let mut password = String::with_capacity(8);
    for i in 4000000u32.. {
        let digest = md5::compute(format!("{}{}", INPUT, i));
        let digest = format!("{:?}", digest);
        if digest.starts_with("00000") {
            password.push(*digest.as_bytes().get(5).unwrap() as char);
            println!("{} at {}", password, i);
            if password.len() > 7 { break; }
        }
    }
}

fn get_index(digest: &str) -> Option<usize> {
    let c = digest.chars().nth(5).unwrap();
    let digit = c.to_digit(16).unwrap() as usize;
    if digit < 8 {
        Some(digit)
    } else {
        None
    }
}

fn get_char(digest: &str) -> char {
    digest.chars().nth(6).unwrap()
}

fn part2() {
    println!("\nPart 2: ");
    let mut password = ['-'; 8];
    let mut count = 8;
    for i in 0u32.. {
        let digest = md5::compute(format!("{}{}", INPUT, i));
        let digest = format!("{:?}", digest);
        if digest.starts_with("00000") {
            if let Some(index) = get_index(&digest) {
                if password[index] != '-' { continue; }
                password[index] = get_char(&digest);
                println!("{}", String::from_iter(password));
                count -= 1; if count == 0 { break; }
            }
        }
    }
}

fn main()  {
    part1();
    part2();
}