use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Receiver {
    Bot(usize),
    Output(usize),
}

#[derive(Debug, Clone)]
pub struct Bot {
    pub id: usize,
    pub gives_low_to: Receiver,
    pub gives_high_to: Receiver,
    pub recieves_from: Vec<usize>,
    pub values: Vec<u32>,
}

impl Bot {
    pub fn new(id: usize, gives_low_to: Receiver, gives_high_to: Receiver) -> Self {
        Bot {
            id,
            gives_low_to,
            gives_high_to,
            recieves_from: vec![],
            values: vec![],
        }
    }
}

#[derive(Debug)]
pub struct BotComputer {
    pub bots: Vec<Bot>,
    pub outputs: [Option<u32>; 21],
}

impl BotComputer {
    pub fn from_str(text: &str) -> Self {
        let mut bots: Vec<Bot> = Vec::new();

        // first pass: create bot and add low_to high_to
        for line in text.lines() {
            let tokens = line.split_ascii_whitespace().collect_vec();
            // bot 7 gives low to bot 4 and high to bot 80
            if tokens[0] == "bot" {
                let id: usize = tokens[1].parse().unwrap();
                let low_to: usize = tokens[6].parse().unwrap();
                let high_to: usize = tokens[11].parse().unwrap();
                let low_to = if tokens[5] == "bot" {
                    Receiver::Bot(low_to)
                } else {
                    Receiver::Output(low_to)
                };
                let high_to = if tokens[10] == "bot" {
                    Receiver::Bot(high_to)
                } else {
                    Receiver::Output(high_to)
                };
                bots.push(Bot::new(id, low_to, high_to))
            }
        }

        // second pass: add initial values if bot has any
        bots.sort_by_key(|bot| bot.id);
        for line in text.lines() {
            let tokens = line.split_ascii_whitespace().collect_vec();
            // value 5 goes to bot 3
            if tokens[0] == "value" {
                let id: usize = tokens[5].parse().unwrap();
                let value: u32 = tokens[1].parse().unwrap();
                bots[id].values.push(value);
            }
        }

        // third pass: fill receive_from lists
        for bot in bots.clone().into_iter() {
            if let Receiver::Bot(id) = bot.gives_high_to {
                bots[id].recieves_from.push(bot.id);
            }
            if let Receiver::Bot(id) = bot.gives_low_to {
                bots[id].recieves_from.push(bot.id);
            }
        }

        BotComputer {
            bots,
            outputs: [None; 21],
        }
    }

    pub fn get_values(&mut self, id: usize) -> (u32, u32) {
        if self.bots[id].values.len() == 2 {
            return (self.bots[id].values[0], self.bots[id].values[1]);
        }

        for giver_id in self.bots[id].recieves_from.clone() {
            let given_values = self.get_values(giver_id);
            match self.bots[giver_id].gives_high_to {
                Receiver::Bot(receiver_id) => {
                    if receiver_id == id {
                        self.bots[id].values.push(given_values.0.max(given_values.1));
                    }
                }
                Receiver::Output(receiver_id) => {
                    self.outputs[receiver_id] = Some(given_values.0.max(given_values.1));
                }
            }
            match self.bots[giver_id].gives_low_to {
                Receiver::Bot(receiver_id) => {
                    if receiver_id == id {
                        self.bots[id].values.push(given_values.0.min(given_values.1));
                    }
                }
                Receiver::Output(receiver_id) => {
                    self.outputs[receiver_id] = Some(given_values.0.min(given_values.1));
                }
            }
        }

        (self.bots[id].values[0], self.bots[id].values[1])
    }
}
