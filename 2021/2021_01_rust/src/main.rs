const INPUT: &str = include_str!("..\\input.txt");

fn part1(measurements: &[i32]) {
    let mut count = 0;
    for values in measurements.windows(2) {
        if values[1] > values[0] {
            count += 1;
        }
    }
    println!("Part 1: {}", count);
}

fn part2(measurements: &[i32]) {
    let mut count = 0;
    for values in measurements.windows(4) {
        if (values[0] + values[1] + values[2]) < (values[1] + values[2] + values[3]) {
            count += 1;
        }
    }
    println!("Part 2: {}", count);
}

fn main()  {
    let measurements: Vec<i32> = INPUT.lines().map(|x|x.parse().unwrap()).collect();
    part1(&measurements);
    part2(&measurements);
}