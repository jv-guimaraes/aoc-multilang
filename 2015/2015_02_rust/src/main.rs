use std::fs;

fn read_line(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn problem_1(text: &str) -> u32 {
    let rects = text.lines().map(|x| x.split('x').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>());
    let mut total = 0;
    for rect in rects {
        let l = rect[0];
        let h = rect[1];
        let w = rect[2];
        total += 2*l*w + 2*w*h + 2*h*l + (l*w).min(w*h).min(h*l);
    }
    total
}

fn problem_2(text: &str) -> u32 {
    let rects = text.lines().map(|x| x.split('x').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>());
    let mut total = 0;
    for mut rect in rects {
        rect.sort();
        total += rect[0] * 2 + rect[1] * 2 + rect.iter().product::<u32>();
    }
    total
}

fn main() {
    let text = read_line("input.txt");
    println!("Problem 1: {}", problem_1(&text));
    println!("Problem 2: {}", problem_2(&text));
}