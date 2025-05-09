#![allow(clippy::needless_range_loop)]
#![feature(slice_group_by)]

mod parsing;

use std::{fmt::Debug, collections::HashMap};

use itertools::Itertools;
use parsing::*;

const INPUT: &str = include_str!("..\\input.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mind {
    Sleeping,
    Awake,
    Flip,
}
impl Mind {
    fn flip(self) -> Mind {
        match self {
            Mind::Sleeping => Mind::Awake,
            Mind::Awake => Mind::Sleeping,
            Mind::Flip => panic!("Can't flip Mind::Flip!"),
        }
    }
}

impl Debug for Mind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sleeping => write!(f, "#"),
            Self::Awake => write!(f, "."),
            Self::Flip => write!(f, "+"),
        }
    }
}

#[derive(Debug)]
struct Day {
    id: u32,
    minutes: [Mind; 60],
}

impl Day {
    fn new(id: u32) -> Day {
        Day { id, minutes: [Mind::Awake; 60] }
    }

    fn total_sleep(&self) -> u32 {
        self.minutes
            .iter()
            .filter(|x| **x == Mind::Sleeping)
            .count() as u32
    }
}

fn calculate_days(logs: &[Log]) -> Vec<Day> {
    let mut days: Vec<Day> = Vec::new();
    // First pass
    for days_log in logs.group_by(|a, b| a.day == b.day) {
        let mut day = Day::new(days_log[0].action.id());
        for log in days_log {
            match log.action {
                Action::BeginShift(_) => (),
                Action::FallSleep => day.minutes[log.time as usize] = Mind::Flip,
                Action::WakeUp => day.minutes[log.time as usize] = Mind::Flip,
            }
        }
        days.push(day);
    }

    // Second pass
    for day in days.iter_mut() {
        let mut mind = Mind::Awake;
        for minute in day.minutes.iter_mut() {
            if let Mind::Flip = minute {
                mind = mind.flip();
            }
            *minute = mind;
        }
    }

    days
}

fn part1() {
    let logs = INPUT.lines().map(Log::new).collect_vec();
    let days = calculate_days(&logs);
    // days.iter().for_each(|x| println!("{:?}", x.minutes_to_str()));
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for day in days.iter() {
        counter.entry(day.id)
            .and_modify(|e| *e += day.total_sleep())
            .or_insert(day.total_sleep());
    }
    // println!("{:?}", counter);
    let id = counter.into_iter().max_by_key(|x| x.1).unwrap().0;
    let select_days = days.iter().filter(|x| x.id == id).collect_vec();
    let mut minute_sum = [0; 60];
    for day in select_days {
        for i in 0..60 {
            minute_sum[i] += if day.minutes[i] == Mind::Sleeping { 1 } else { 0 };
        }
    }
    let sleepiest_minute = minute_sum.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
    println!("Part 1: {} x {} = {}", id, sleepiest_minute, id * sleepiest_minute as u32);
}

fn part2() {
    let logs = INPUT.lines().map(Log::new).collect_vec();
    let days = calculate_days(&logs);
    let mut days_by_id: HashMap<u32, Vec<&Day>> = HashMap::new();
    for day in days.iter() {
        days_by_id.entry(day.id).and_modify(|x| x.push(day)).or_insert(vec![day]);
    }
    let mut records: Vec<(u32, (usize, i32))> = Vec::new();
    for id in days_by_id.keys() {
        let mut minute_sum = [0; 60];
        for day in days_by_id.get(id).unwrap() {
            for i in 0..60 {
                minute_sum[i] += if day.minutes[i] == Mind::Sleeping { 1 } else { 0 };
            }
        }
        let sleepiest_minute = minute_sum.iter().enumerate().max_by_key(|x| x.1).unwrap();
        records.push((*id, (sleepiest_minute.0, *sleepiest_minute.1)));
    }
    let solution = records.into_iter().max_by_key(|x| x.1.1).unwrap();
    println!("Part 2: {:?}", solution.0 * solution.1.0 as u32);
}

fn main() {
    part1();
    part2();
}
