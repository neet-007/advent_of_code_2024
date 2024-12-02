use std::fs;

fn main() {
    part2();
}

fn part2() {
    let content = fs::read_to_string("src/input.txt").expect("Failed to read file");
    let mut safe_count = 0;

    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            if is_safe(&numbers) {
                safe_count += 1;
            } else {
                for i in 0..numbers.len() {
                    let mut modified_numbers = numbers.clone();
                    modified_numbers.remove(i);
                    if is_safe(&modified_numbers) {
                        safe_count += 1;
                        return;
                    }
                }
            }
        });

    println!("Number of safe reports: {}", safe_count);
}

fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let direction = numbers[1] - numbers[0];
    if direction == 0 || direction.abs() > 3 {
        return false;
    }

    for window in numbers.windows(2) {
        let diff = window[1] - window[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if (direction > 0 && diff < 0) || (direction < 0 && diff > 0) {
            return false;
        }
    }

    true
}

fn part1() {
    let content = fs::read_to_string("src/input.txt").expect("Failed to read file");

    let mut count = 0;

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .for_each(|(line_number, line)| {
            match line
                .split_whitespace()
                .map(|num| num.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
            {
                Ok(numbers) => {
                    let initial = (true, 0);
                    let result = numbers
                        .windows(2)
                        .fold(initial, |(flag, direction), window| {
                            if !flag {
                                return (false, direction);
                            }

                            let diff = window[1] - window[0];
                            if diff.abs() < 1 || diff.abs() > 3 {
                                return (false, direction);
                            }
                            let mut new_direction = direction;
                            if direction == 0 {
                                new_direction = if diff > 0 {
                                    1
                                } else if diff < 0 {
                                    -1
                                } else {
                                    0
                                };
                            }

                            let new_flag =
                                if new_direction > 0 && diff < 0 || new_direction < 0 && diff > 0 {
                                    false
                                } else {
                                    true
                                };

                            (new_flag, new_direction)
                        });
                    if result.0 {
                        count += 1;
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing numbers on line {}: {}", line_number + 1, e);
                }
            }
        });

    println!("count {}", count);
}
