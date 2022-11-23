use std::ops::Range;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let range: Range<i64> = 1..99999999999999;

    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for n in range.rev() {
        let input = n.to_string();

        if !input.contains("0") {
            let solution = solve(&input);

            if solution == 0 {
                println!("Found the magic input: {input}");
            }
        }

        if n % 1000000 == 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            println!("Currently at {n}");

            if now - start > 30 {
                println!("Done running for 30 seconds");
                break;
            }
        }
    }
}

fn solve_block(w: i64, z: i64, div: i64, offset_1: i64, offset_2: i64) -> i64 {
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / div;
    let x = x + offset_1;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + offset_2;
    let y = y * x;
    let z = z + y;

    return z;
}

fn solve(input: &str) -> i64 {
    let mut input: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .rev()
        .collect();

    let z = solve_block(input.pop().unwrap(), 0, 1, 13, 14);
    let z = solve_block(input.pop().unwrap(), z, 1, 25, 8);
    let z = solve_block(input.pop().unwrap(), z, 1, 12, 8);
    let z = solve_block(input.pop().unwrap(), z, 1, 11, 5);
    let z = solve_block(input.pop().unwrap(), z, 26, 0, 4);
    let z = solve_block(input.pop().unwrap(), z, 1, 15, 10);
    let z = solve_block(input.pop().unwrap(), z, 26, -13, 13);
    let z = solve_block(input.pop().unwrap(), z, 1, 10, 16);
    let z = solve_block(input.pop().unwrap(), z, 26, -9, 5);
    let z = solve_block(input.pop().unwrap(), z, 1, 11, 6);
    let z = solve_block(input.pop().unwrap(), z, 1, 13, 13);
    let z = solve_block(input.pop().unwrap(), z, 26, -14, 6);
    let z = solve_block(input.pop().unwrap(), z, 26, -3, 7);
    let z = solve_block(input.pop().unwrap(), z, 26, -2, 13);
    let z = solve_block(input.pop().unwrap(), z, 26, -14, 3);

    return z;
}
