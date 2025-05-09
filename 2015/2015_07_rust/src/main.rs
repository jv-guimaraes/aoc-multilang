// #![allow(unused)]

use std::{collections::HashMap};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref DICT: Mutex<HashMap<String, u16>> = Mutex::new(HashMap::new());
}

fn insert_into_dict(identifier: &str, value: u16) {
    let mut dict = DICT.lock().unwrap();
    dict.insert(identifier.to_owned(), value);
    // println!("{} = {} ({})", identifier, value, dict.len());
}

fn value_of(commands: &str, identifier: &str) -> u16 {
    if DICT.lock().unwrap().contains_key(identifier) {
        return *DICT.lock().unwrap().get(identifier).unwrap();
    }

    for line in commands.lines().rev() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        
        if *tokens.last().unwrap() != identifier { continue; }
        
        if tokens.len() == 3 {
            let a = tokens[0].parse().unwrap_or_else(|_| value_of(commands, tokens[0]));
            insert_into_dict(identifier, a);
            return  a;
        }

        else if tokens.len() == 4  { // NOT
            let a = tokens[1];
            let a = !a.parse().unwrap_or_else(|_| value_of(commands, a));
            insert_into_dict(identifier, a);
            return a;
        }

        let operator = tokens[1];
        let (a, b) = (tokens[0], tokens[2]);
        let a = a.parse().unwrap_or_else(|_| value_of(commands, a));
        let b = b.parse().unwrap_or_else(|_| value_of(commands, b));
        
        let value = match operator {
            "AND" => a & b,
            "OR" => a | b,
            "LSHIFT" => a << b,
            "RSHIFT" => a >> b,
            op => panic!("Invalid operator: {op}"),
        };

        insert_into_dict(identifier, value);
        return value;
    }
    
    panic!("Yabe!");
}

fn main() {
    let commands = include_str!("..\\input.txt");
    println!("Solution 1: {}", value_of(commands, "a"));
    
    DICT.lock().unwrap().clear();
    DICT.lock().unwrap().insert("b".to_owned(), 956);
    println!("Solution 2: {}", value_of(commands, "a"));
}
