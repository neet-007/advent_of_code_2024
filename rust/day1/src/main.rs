use std::{collections::HashMap, fs, iter::zip};

fn main() {
    match fs::read_to_string("src/input.txt") {
        Ok(content) => {
            let (mut left_numbers, mut right_numbers): (Vec<i32>, Vec<i32>) = content
                .lines()
                .filter_map(|line| {
                    let mut parts = line.split_whitespace();
                    match (
                        parts.next()?.parse::<i32>().ok(),
                        parts.next()?.parse::<i32>().ok(),
                    ) {
                        (Some(left), Some(right)) => Some((left, right)),
                        _ => None,
                    }
                })
                .unzip();

            left_numbers.sort_unstable();
            right_numbers.sort_unstable();

            let mut freq_map: HashMap<i32, usize> = HashMap::new();

            for &num in &right_numbers {
                *freq_map.entry(num).or_insert(0) += 1;
            }

            let result: i32 = left_numbers
                .iter()
                .map(|&num| num * freq_map.get(&num).copied().unwrap_or(0) as i32)
                .sum();

            println!("sum: {}", result);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
