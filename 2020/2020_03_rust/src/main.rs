use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

fn count_trees(x: usize, y: usize) -> usize {
    let grid = INPUT.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let len = grid[0].len();
    let mut tree_count = 0;
    let mut pos = (0, 0);
    loop {
        pos.0 = (pos.0 + x) % len;
        pos.1 += y;
        if pos.1 >= grid.len() {
            break;
        }
        if grid[pos.1][pos.0] == '#' {
            tree_count += 1;
        }
    }
    tree_count
}

fn main() {
    println!("Part 1: {}", count_trees(3, 1));

    // Part 2
    let mut total_mult = 1;
    for (x, y) in [(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)] {
        total_mult *= count_trees(x, y);
    }
    println!("Part2: {}", total_mult);
}
