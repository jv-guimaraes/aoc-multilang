mod cord;
mod p1;
mod p2;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    let mut matrix = p1::start_matrix();
    
    for command in input.lines() {
        p1::run(&mut matrix, command);
    }

    println!("Problem 1: {}", p1::light_count(&matrix));

    /*-------------------------------------------------------------- */
    let mut matrix = p2::start_matrix();
   
    for command in input.lines() {
        p2::run(&mut matrix, command);
    }

    println!("Problem 2: {}", p2::total_brightness(&matrix));
}
