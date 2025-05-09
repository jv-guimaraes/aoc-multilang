use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use once_cell::sync::Lazy;

const INPUT: &str = include_str!("..\\input.txt");

#[derive(Debug)]
struct Passport<'a> {
    birth_year: u16,
    issue_year: u16,
    expiration_year: u16,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    passport_id: &'a str,
}

impl Passport<'_> {
    fn from_str(text: &str) -> Passport {
        let text_entries = text.split_ascii_whitespace().collect_vec();
        let mut atributes: HashMap<&str, &str> = HashMap::new();
        for entry in text_entries {
            let (key, value) = entry.split(':').collect_tuple().unwrap();
            atributes.insert(key, value);
        }
        Passport {
            birth_year: atributes.get("byr").unwrap().parse().unwrap(),
            issue_year: atributes.get("iyr").unwrap().parse().unwrap(),
            expiration_year: atributes.get("eyr").unwrap().parse().unwrap(),
            height: atributes.get("hgt").unwrap(),
            hair_color: atributes.get("hcl").unwrap(),
            eye_color: atributes.get("ecl").unwrap(),
            passport_id: atributes.get("pid").unwrap(),
        }
    }

    fn is_height_valid(&self) -> bool {
        if !self.height.ends_with("in") && !self.height.ends_with("cm") {
            return false;
        }
        let num: u16 = self.height[0..self.height.len()-2].parse().unwrap();
        let unit: &str = &self.height[(self.height.len()-2)..];
        match unit {
            "cm" => (150..=193).contains(&num),
            "in" => (59..=76).contains(&num),
            _ => false,
        }
    }
    
    fn is_hair_color_valid(&self) -> bool {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]").unwrap());
        RE.is_match(self.hair_color)
    }

    fn is_eye_color_valid(&self) -> bool {
        let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valid_colors.contains(&self.eye_color)
    }
    
    fn is_passport_id_valid(&self) -> bool {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d\d\d\d\d\d\d\d\d)$").unwrap());
        RE.is_match(self.passport_id)
    }
    
    fn is_valid(&self) -> bool {
        if !(1920..=2002).contains(&self.birth_year) {
            return false;
        }
        if !(2010..=2020).contains(&self.issue_year) {
            return false;
        }
        if !(2020..=2030).contains(&self.expiration_year) {
            return false;
        }
        if !self.is_height_valid() {
            return false;
        }
        if !self.is_hair_color_valid() {
            return  false;
        }
        if !self.is_eye_color_valid() {
            return false;
        }
        if !self.is_passport_id_valid() {
            return false;
        }
        true
    }
}

fn are_fields_present(text: &str) -> bool {
    let num_of_entries = text.matches(':').count();
    if num_of_entries < 7 {
        return false;
    }
    if num_of_entries == 7 && text.contains("cid:") {
        return false;
    }
    true
}

fn main() {
    println!("Part 1: {}", INPUT.split("\r\n\r\n").filter(|x| are_fields_present(x)).count());

    let batch = INPUT.split("\r\n\r\n").filter(|x| are_fields_present(x));
    let batch = batch.into_iter().map(Passport::from_str).collect_vec();
    println!("Part 2: {:?}", batch.iter().filter(|x| x.is_valid()).count());
}
