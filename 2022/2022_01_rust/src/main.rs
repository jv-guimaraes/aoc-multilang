fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let elfs = input.split("\n\n").map(|x| x.trim().split_ascii_whitespace());
    let elfs = elfs.map(|elf| elf.map(|x| x.parse::<i32>().unwrap()));
    
    let mut sums: Vec<_> = elfs.map(|elf| elf.sum::<i32>()).collect();
    sums.sort_by(|a, b| b.partial_cmp(a).unwrap());
   
    println!("Solution 1: {}", sums[0]);
    println!("Solution 2: {}", sums.iter().take(3).sum::<i32>());
}
