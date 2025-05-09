#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    ROCK = 1,
    PAPER,
    SCISSOR,
}

impl Hand {
    fn new(play_str: &str) -> Hand {
        match play_str {
            "X" | "A" => Hand::ROCK,
            "Y" | "B" => Hand::PAPER,
            "Z" | "C" => Hand::SCISSOR,
            _ => panic!("Can't form play from incorrect string!"),
        }
    }

    fn battle(self, other: Self) -> i32 {
        if self == other { return 3 + self.score() };
        match (self, other) {
            (Hand::ROCK, Hand::SCISSOR) => 6 + self.score(),
            (Hand::PAPER, Hand::ROCK) => 6 + self.score(),
            (Hand::SCISSOR, Hand::PAPER) => 6 + self.score(),
            _ => self.score(),
        }
    }

    fn score(self) -> i32 {
        self as i32
    }

    fn wins_against(self) -> Hand {
        match self {
            Hand::ROCK => Hand::SCISSOR,
            Hand::PAPER => Hand::ROCK,
            Hand::SCISSOR => Hand::PAPER,
        }
    }

    fn loses_against(self) -> Hand {
        match self {
            Hand::ROCK => Hand::PAPER,
            Hand::PAPER => Hand::SCISSOR,
            Hand::SCISSOR => Hand::ROCK,
        }
    }

    fn draws_against(self) -> Hand {
        self
    }
}

enum PlayerResult {
    WIN,
    LOSS,
    DRAW
}

impl PlayerResult {
    fn new(result: &str) -> PlayerResult {
        match result {
            "X" => PlayerResult::LOSS,
            "Y" => PlayerResult::DRAW,
            "Z" => PlayerResult::WIN,
            _ => panic!("Invalid round result!")
        }
    }
}

fn main() {
    let input = include_str!("..\\input.txt");
    let rounds: Vec<_> = input.lines().map(|x| x.split(" ").collect::<Vec<_>>()).collect();
    
    let mut total_score_1 = 0;
    for round in rounds.iter() {
        let player = Hand::new(round[1]);
        let enemy = Hand::new(round[0]);
        total_score_1 += player.battle(enemy);
    }
    println!("Solution 1: {total_score_1}");

    let mut total_score_2 = 0;
    for round in rounds.iter() {
        let enemy = Hand::new(round[0]);
        let result = PlayerResult::new(round[1]);
        total_score_2 += match result {
            PlayerResult::WIN => enemy.loses_against().battle(enemy),
            PlayerResult::LOSS =>  enemy.wins_against().battle(enemy),
            PlayerResult::DRAW =>  enemy.draws_against().battle(enemy),
        }
    }
    println!("Solution 2: {total_score_2}")
}
