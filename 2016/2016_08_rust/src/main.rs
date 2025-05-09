mod screen;

use screen::Screen;

const INPUT: &str = include_str!("..\\input.txt");

fn main()  {
    let mut screen = Screen::new();
    for command in INPUT.lines() {
        screen.execute(command);
    }
    println!("{:?}", screen);
    println!("Pixels turned on: {:?}", screen.on_pixels());
}