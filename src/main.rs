fn main() {
    let solutions = find_all_solutions();

    for solution in solutions {
        let is_valid = calculate_z(&solution) == 0;

        println!("Solution: {solution} is_valid={is_valid}");
    }
}

fn find_all_solutions() -> Vec<String> {
    let mut solutions = Vec::new();

    for (n1, n14) in solve_pair(13, 14, -14, 3) {
        for (n2, n13) in solve_pair(12, 8, -2, -13) {
            for (n3, n4) in solve_pair(11, 5, 0, 4) {
                for (n5, n6) in solve_pair(15, 10, -13, 13) {
                    for (n7, n8) in solve_pair(10, 16, -9, 5) {
                        for (n9, n12) in solve_pair(11, 6, -3, 7) {
                            for (n10, n11) in solve_pair(13, 13, -14, 6) {
                                let solution = format!(
                                    "{n1}{n2}{n3}{n4}{n5}{n6}{n7}{n8}{n9}{n10}{n11}{n12}{n13}{n14}"
                                );

                                solutions.push(solution);
                            }
                        }
                    }
                }
            }
        }
    }

    return solutions;
}

fn calculate_z(input: &str) -> i64 {
    let mut input: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .rev()
        .collect();

    let z = calculate_z_block(input.pop().unwrap(), 0, 1, 13, 14);   // push 1
    let z = calculate_z_block(input.pop().unwrap(), z, 1, 12, 8);    // push 2
    let z = calculate_z_block(input.pop().unwrap(), z, 1, 11, 5);    // push 3
    let z = calculate_z_block(input.pop().unwrap(), z, 26, 0, 4);    // pop 3
    let z = calculate_z_block(input.pop().unwrap(), z, 1, 15, 10);   // push 4
    let z = calculate_z_block(input.pop().unwrap(), z, 26, -13, 13); // pop 4
    let z = calculate_z_block(input.pop().unwrap(), z, 1, 10, 16);   // push 5
    let z = calculate_z_block(input.pop().unwrap(), z, 26, -9, 5);   // pop 5
    let z = calculate_z_block(input.pop().unwrap(), z, 1, 11, 6);    // push 6
    let z = calculate_z_block(input.pop().unwrap(), z, 1, 13, 13);   // push 7
    let z = calculate_z_block(input.pop().unwrap(), z, 26, -14, 6);  // pop 7
    let z = calculate_z_block(input.pop().unwrap(), z, 26, -3, 7);   // pop 6
    let z = calculate_z_block(input.pop().unwrap(), z, 26, -2, 13);  // pop 2
    let z = calculate_z_block(input.pop().unwrap(), z, 26, -14, 3);  // pop 1

    return z;
}

fn calculate_z_block(input: i64, stack: i64, div: i64, offset_1: i64, offset_2: i64) -> i64 {
    let peek = stack % 26 + offset_1;

    return if peek == input {
        stack / div
    } else {
        stack / div * 26 + input + offset_2
    };
}

fn solve_pair(
    push_offset_1: i64,
    push_offset_2: i64,
    pop_offset_1: i64,
    pop_offset_2: i64,
) -> Vec<(i64, i64)> {
    let mut options = Vec::new();

    for i in 1..=9i64 {
        let z = calculate_z_block(i, 0, 1, push_offset_1, push_offset_2);

        for o in 1..=9i64 {
            let z = calculate_z_block(o, z, 26, pop_offset_1, pop_offset_2);

            if z == 0 {
                options.push((i, o));
            }
        }
    }

    return options;
}
#[cfg(test)]
mod tests {
    use crate::{calculate_z, find_all_solutions};

    #[test]
    fn all_solutions_are_valid() {
        let solutions = find_all_solutions();

        for solution in solutions {
            assert_eq!(calculate_z(&solution), 0);
        }
    }
}
