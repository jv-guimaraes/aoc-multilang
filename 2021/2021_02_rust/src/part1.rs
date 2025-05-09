use crate::INPUT;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_pos(&mut self, command: &str) {
        let command: Vec<&str> = command.split(' ').collect();
        let (direction, distance) = (command[0], command[1].parse::<i32>().unwrap());
        if direction == "forward" {
            self.x += distance;
        }
        else if direction == "down" {
            self.y += distance;
        } else {
            self.y -= distance;
        }
    }
}

pub fn run()  {
    let mut pos = Pos {x: 0, y: 0};
    for command in INPUT.lines() {
        pos.move_pos(command);
    }
    println!("Part 1: {}", pos.x * pos.y);
}