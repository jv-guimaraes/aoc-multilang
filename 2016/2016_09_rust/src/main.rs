use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

fn find_closing_parentheses(chars: &[char], opening_ix: usize) -> Option<usize> {
    ((opening_ix+1)..).find(|&j| chars[j] == ')')
}

fn decompress(text: &str) -> String {
    let mut result = String::new();
    let chars = text.chars().collect_vec();
    
    let mut i = 0;
    while i < chars.len() {
        // Not a compression sequence
        if chars[i] != '(' {
            result.push(chars[i]);
            i += 1;
            continue;
        }
        
        // Find closing parentheses
        let closing_parentheses_ix = find_closing_parentheses(&chars, i).unwrap();

        // Calculate sequence size and range and the times to repeat
        let marker = &text[i+1..closing_parentheses_ix];
        let range_size: usize = marker.split('x').next().unwrap().parse().unwrap();
        let range = (closing_parentheses_ix+1)..(closing_parentheses_ix+1+range_size);
        let times: usize  = marker.split('x').nth(1).unwrap().parse().unwrap();

        // Write slices to result string
        for _ in 0..times {
            result.push_str(&text[range.clone()]);
        }

        // Calculate the next i
        i = closing_parentheses_ix + 1 + range_size;
    }
    result
}

fn deep_decompress(text: &str) -> usize {
    let mut total = 0;
    let chars = text.chars().collect_vec();

    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '(' {
            i += 1;
            total += 1;
            continue;
        }

        // Find closing parentheses
        let closing_parentheses_ix = find_closing_parentheses(&chars, i).unwrap();

        // Calculate sequence size and range and the times to repeat
        let marker = &text[i+1..closing_parentheses_ix];
        let range_size: usize = marker.split('x').next().unwrap().parse().unwrap();
        let range = (closing_parentheses_ix+1)..(closing_parentheses_ix+1+range_size);
        let times: usize  = marker.split('x').nth(1).unwrap().parse().unwrap();

        total += times * deep_decompress(&text[range.clone()]);

        // Calculate the next i
        i = closing_parentheses_ix + 1 + range_size;
    }

    total
}

fn main()  {
    println!("Part 1: {}", decompress(INPUT.trim()).len());
    println!("Part 2: {}", deep_decompress(INPUT.trim()));
}

#[test]
fn deep_no_compression_sequence_test() {
    assert_eq!("ADVENT".len(), deep_decompress("ADVENT"));
}

#[test]
fn deep_compression_test() {
    assert_eq!(deep_decompress("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY".len());
    assert_eq!(deep_decompress("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(deep_decompress("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
}

#[test]
fn no_compression_sequence_test() {
    assert_eq!("ADVENT".to_owned(), decompress("ADVENT"));
}

#[test]
fn compression_sequence_whitout_inner_marker() {
    assert_eq!("ABBBBBC".to_owned(), decompress("A(1x5)BC"));
    assert_eq!("XYZXYZXYZ".to_owned(), decompress("(3x3)XYZ"));
    assert_eq!("ABCBCDEFEFG".to_owned(), decompress("A(2x2)BCD(2x2)EFG"));
}

#[test]
fn compression_with_inner_marker() {
    assert_eq!("(1x3)A".to_owned(), decompress("(6x1)(1x3)A"));
    assert_eq!("X(3x3)ABC(3x3)ABCY".to_owned(), decompress("X(8x2)(3x3)ABCY"));
}