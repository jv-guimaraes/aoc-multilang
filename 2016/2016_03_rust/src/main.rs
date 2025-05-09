fn is_valid_triangle(triangle: &[i32]) -> bool {
    if triangle[0] >= triangle[1] + triangle[2] { return false; }
    if triangle[1] >= triangle[0] + triangle[2] { return false; }
    if triangle[2] >= triangle[0] + triangle[1] { return false; }
    true
}

fn part1(input: &str) {
    let mut valid_triangles = 0;
    for triangle in input.lines() {
        let sides: Vec<i32> = triangle
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

            if is_valid_triangle(&sides) {
                valid_triangles += 1;
            } 
    }
    println!("Part 1: {valid_triangles}");
}

fn part2(input: &str) {
    let input: Vec<i32> = input.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut valid_triangles = 0;
    
    for i in (0..input.len()).step_by(9) {
        for j in 0..3 {
            let sides = vec![input[i+j], input[i+3+j], input[i+6+j]];
            if is_valid_triangle(&sides) {
                valid_triangles += 1;
            } 
        }
    }
    
    println!("Part 2: {valid_triangles}");
}

fn main() {
    let input = include_str!("..\\input.txt");
    part1(input);
    part2(input);
}
