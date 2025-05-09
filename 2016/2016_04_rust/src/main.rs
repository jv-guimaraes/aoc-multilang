use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use lazy_static::lazy_static;
use regex::Regex;
use itertools::*;

const INPUT: &str = include_str!("..\\input.txt");

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(Debug)]
struct Room<'a> {
    name: &'a str,
    id: u32,
    checksum: &'a str,
}

impl Display for Room<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}|{} {:?}", self.name, self.id, self.checksum, self.is_real())
    }
}

impl Room<'_> {
    fn from_str(text: &str) -> Room {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"^([a-z]+-)+").unwrap();
            static ref ID_RE: Regex = Regex::new(r"\d+").unwrap();
            static ref CHECKSUM_RE: Regex = Regex::new(r"\w+\]").unwrap();
        }
        let name = NAME_RE.find(text).unwrap().as_str();
        let id = ID_RE.find(text).unwrap().as_str().parse().unwrap();
        let checksum = CHECKSUM_RE
            .find(text)
            .unwrap()
            .as_str()
            .strip_suffix(']')
            .unwrap();
        Room { name, id, checksum }
    }

    fn letter_count(&self) -> HashMap<char, usize> {
        self.name.chars().filter(|c| c.is_ascii_lowercase()).counts()
    }

    fn is_real(&self) -> bool {
        let mut vetor = self.letter_count().into_iter().collect::<Vec<_>>();
        vetor.sort_by_key(|a| a.0);
        vetor.sort_by_key(|a| std::cmp::Reverse(a.1));
        let vetor: Vec<char> = vetor.into_iter().take(5).map(|x| x.0).collect();
        let checksum: Vec<char> = self.checksum.chars().collect();
        vetor == checksum
    }

    fn decrypt(&self) -> String {
        let mut result = String::new();
        for byte in self.name.bytes() {
            if (byte as char) == '-' {
                result.push(' ');
                continue;
            }
            let index = ((byte - 97) as usize + self.id as usize) % 26;
            let decrypted_char = ALPHABET[index];
            result.push(decrypted_char);
        }
        result
    }
}

fn main() {
    // Part 1
    let rooms: Vec<Room> = INPUT.lines().map(Room::from_str).collect();
    let checksum: u32 = rooms.iter().filter(|r| r.is_real()).fold(0, |acc, x| acc + x.id);
    println!("Part 1: {checksum}");

    // Part 2
    let valid_rooms: Vec<&Room> = rooms.iter().filter(|r| r.is_real()).collect();
    let northpole_room = valid_rooms.iter().find(|x| x.decrypt().contains("northpole")).unwrap();
    println!("Part 2: {}", northpole_room.id);
}
