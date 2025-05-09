use crate::INPUT;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    aim: i32,
}

impl Pos {
    fn move_pos(&mut self, command: &str) {
        let command: Vec<&str> = command.split(' ').collect();
        let (direction, distance) = (command[0], command[1].parse::<i32>().unwrap());
        if direction == "forward" {
            self.x += distance;
            self.y += self.aim * distance;
        }
        else if direction == "down" {
            self.aim += distance;
        } else {
            self.aim -= distance;
        }
    }
}

pub fn run()  {
    let mut pos = Pos {x: 0, y: 0, aim: 0};
    for command in INPUT.lines() {
        pos.move_pos(command);
    }
    println!("Part 2: {}", pos.x * pos.y);
}