use itertools::Itertools;
use regex_macro::regex;

const INPUT: &str = include_str!("..\\input.txt");

fn supports_tls(text: &str) -> bool {
    let re = regex!(r"(\[.*?\])");
    for find in re.find_iter(text) {
        let slice = &text[find.start()+1..find.end()-1];
        if supports_tls(slice) {
            return false;
        }
    }
    for (a, b, c, d) in text.bytes().tuple_windows() {
        if a != b && a == d && b == c {
            return true;
        }
    }
    false
}

fn supernet_hypernet_split(text: &str) -> (String, String) {
    let re = regex!(r"(\[.*?\])");
    let supernet = re.replace_all(text, "--").to_string();
    let mut hypernet = String::new();
    for find in re.find_iter(text) {
        let slice = &text[find.start()..find.end()];
        hypernet.push_str(slice);
    }
    (supernet, hypernet)
}

fn supports_ssl(text: &str) -> bool {
    let (supernet, hypernet) = supernet_hypernet_split(text);
    // outside brackets
    for (a, b, c) in supernet.bytes().tuple_windows() {
        if a == c && a != b {
            // whitin brackers
            for (i, j, k) in hypernet.bytes().tuple_windows() {
                if i == b && k == b && j == a {
                    return true;
                }
            }
        }
    }
    false
}

fn main()  {
    let r = INPUT.lines().filter(|s| supports_tls(s)).count();
    println!("Part 1: {}", r);

    let r = INPUT.lines().filter(|s| supports_ssl(s)).count();
    println!("Part 2: {}", r);
}