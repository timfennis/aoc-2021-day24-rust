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

fn solve_block_1(w: i64, z: i64) -> i64 {
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 1;
    let x = x + 13;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 14;
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

    // Block 1
    let z = solve_block_1(input.pop().unwrap(), 0);

    // Block 2
    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 1;
    let x = x + 12;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 8;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 1;
    let x = x + 11;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 5;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 26;
    let x = x + 0;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 4;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 1;
    let x = x + 15;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 10;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 26;
    let x = x + -13;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 13;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 1;
    let x = x + 10;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 16;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 26;
    let x = x + -9;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 5;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 1;
    let x = x + 11;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 6;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 1;
    let x = x + 13;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 13;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 26;
    let x = x + -14;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 6;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 26;
    let x = x + -3;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 7;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 26;
    let x = x + -2;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 13;
    let y = y * x;
    let z = z + y;

    let w = input.pop().unwrap();
    let x = 0;
    let x = x + z;
    let x = x % 26;
    let z = z / 26;
    let x = x + -14;
    let x = if x == w { 1 } else { 0 };
    let x = if x == 0 { 1 } else { 0 };
    let y = 0;
    let y = y + 25;
    let y = y * x;
    let y = y + 1;
    let z = z * y;
    let y = 0;
    let y = y + w;
    let y = y + 3;
    let y = y * x;
    let z = z + y;

    return z;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = crate::solve("93499629698999");
        assert_eq!(result, 0);
    }
}