use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Direction {
    L,
    R,
    T,
    B,
    LT,
    LB,
    RT,
    RB,
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let content: Vec<Vec<String>> = fs::read_to_string("src/input.txt")
        .expect("could not open file")
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_string()).collect())
        .collect();

    let mut count = 0;
    for i in 0..content.len() {
        for j in 0..content[i].len() {
            if content[i][j] == "A" {
                if !(i == 0 || i == content.len() - 1 || j == 0 || j == content[i].len() - 1) {
                    let diag1 = format!(
                        "{}{}{}",
                        content[i - 1][j - 1],
                        content[i][j],
                        content[i + 1][j + 1]
                    );
                    let diag2 = format!(
                        "{}{}{}",
                        content[i + 1][j - 1],
                        content[i][j],
                        content[i - 1][j + 1]
                    );
                    if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("part2: {}", count);
}

fn part1() {
    let content: Vec<Vec<String>> = fs::read_to_string("src/input.txt")
        .expect("could not open file")
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_string()).collect())
        .collect();

    let mut next = HashMap::<&str, &str>::new();
    next.insert("M", "A");
    next.insert("A", "S");
    next.insert("S", "S");

    let mut count = 0;
    for (i, row) in content.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if content[i][j] == "X" {
                if j > 0 {
                    let val = find(&content, i, j - 1, Direction::L, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
                if j > 0 && i + 1 < content.len() {
                    let val = find(&content, i + 1, j - 1, Direction::LB, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
                if j > 0 && i > 0 {
                    let val = find(&content, i - 1, j - 1, Direction::LT, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
                if j + 1 < row.len() {
                    let val = find(&content, i, j + 1, Direction::R, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
                if i > 0 && j + 1 < row.len() {
                    let val = find(&content, i - 1, j + 1, Direction::RT, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
                if i + 1 < content.len() {
                    let val = find(&content, i + 1, j + 1, Direction::RB, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
                if i > 0 {
                    let val = find(&content, i - 1, j, Direction::T, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
                if i + 1 < content.len() {
                    let val = find(&content, i + 1, j, Direction::B, "M", &next, "S");
                    if val == 1 {
                        //println!("match: {}{}", i, j);
                    }
                    count += val;
                }
            }
        }
    }

    println!("part1 {}", count);
}

fn find(
    content: &Vec<Vec<String>>,
    y: usize,
    x: usize,
    dir: Direction,
    pattern: &str,
    next: &HashMap<&str, &str>,
    end: &str,
) -> usize {
    let next_match = next.get(pattern).expect("unexpected pattern");

    if y < content.len() && x < content[y].len() && content[y][x] != pattern {
        return 0;
    }

    if pattern == end {
        return 1;
    }

    match dir {
        Direction::L => {
            if x > 0 {
                find(content, y, x - 1, dir, next_match, next, end)
            } else {
                0
            }
        }
        Direction::R => {
            if x + 1 < content[y].len() {
                find(content, y, x + 1, dir, next_match, next, end)
            } else {
                0
            }
        }
        Direction::T => {
            if y > 0 {
                find(content, y - 1, x, dir, next_match, next, end)
            } else {
                0
            }
        }
        Direction::B => {
            if y + 1 < content.len() {
                find(content, y + 1, x, dir, next_match, next, end)
            } else {
                0
            }
        }
        Direction::LT => {
            if y > 0 && x > 0 {
                find(content, y - 1, x - 1, dir, next_match, next, end)
            } else {
                0
            }
        }
        Direction::LB => {
            if x > 0 && y + 1 < content.len() {
                find(content, y + 1, x - 1, dir, next_match, next, end)
            } else {
                0
            }
        }
        Direction::RT => {
            if y > 0 && x + 1 < content[y].len() {
                find(content, y - 1, x + 1, dir, next_match, next, end)
            } else {
                0
            }
        }
        Direction::RB => {
            if x + 1 < content[y].len() && y + 1 < content.len() {
                find(content, y + 1, x + 1, dir, next_match, next, end)
            } else {
                0
            }
        }
    }
}
