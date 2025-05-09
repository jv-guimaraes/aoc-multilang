fn main() {
    let input = include_str!("..\\input.txt");
    let mut checksum = 0;
    for row in input.lines() {
        let row = row.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        checksum += row.iter().max().unwrap() - row.iter().min().unwrap();
    }
    println!("Part 1: {}", checksum);

    // part 2
    checksum = 0;
    for row in input.lines() {
        let row = row.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        'outer: for i in 0..row.len() {
            for j in 0..row.len() {
                if i == j { continue };
                if row[i] % row[j] == 0 {
                    checksum += row[i].max(row[j]) / row[i].min(row[j]);
                    break 'outer;
                }
            }
        }
    }
    println!("Part 2: {}", checksum);
}
