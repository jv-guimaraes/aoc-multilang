mod bot_computer;

use bot_computer::BotComputer;

const INPUT: &str = include_str!("..\\input.txt");

fn main() {
    let mut bot_computer = BotComputer::from_str(INPUT);

    for id in 0..bot_computer.bots.len() {
        let values = bot_computer.get_values(id);
        if values == (61, 17) || values == (17, 61) {
            println!("Part 1: {}", id);
        }
    }

    println!(
        "Part 2 {}",
        bot_computer.outputs.iter().take(3).map(|x| x.unwrap()).product::<u32>()
    );
}
