use std::{collections::HashMap, fs, usize};

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut content = fs::read_to_string("src/input.txt")
        .expect("Could not read file")
        .split_whitespace()
        .map(|c| c.to_owned())
        .fold(HashMap::<String, usize>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    for _ in 0..75 {
        content = content
            .iter()
            .fold(HashMap::<String, usize>::new(), |mut map, (c, count)| {
                if c == &"0" {
                    *map.entry("1".to_string()).or_insert(0) += count;
                } else if c.len() % 2 == 0 {
                    let left = c[0..c.len() / 2]
                        .parse::<usize>()
                        .expect("Parse left half wrong");
                    let right = c[c.len() / 2..]
                        .parse::<usize>()
                        .expect("Parse right half wrong");

                    *map.entry(left.to_string()).or_insert(0) += count;
                    *map.entry(right.to_string()).or_insert(0) += count;
                } else {
                    let number = c.parse::<usize>().expect("Parse number wrong") * 2024;
                    *map.entry(number.to_string()).or_insert(0) += count;
                }
                map
            });
    }

    println!("part2: {}", content.values().sum::<usize>());
}

fn part1() {
    let mut content = fs::read_to_string("src/input.txt")
        .expect("Could not read file")
        .split_whitespace()
        .map(|c| c.to_owned())
        .collect::<Vec<String>>();

    for _ in 0..25 {
        content = content.iter().fold(Vec::<String>::new(), |mut vec, c| {
            if c == &"0" {
                vec.push("1".to_string());
            } else if c.len() % 2 == 0 {
                let left = c[0..c.len() / 2]
                    .parse::<usize>()
                    .expect("Parse left half wrong");
                let right = c[c.len() / 2..]
                    .parse::<usize>()
                    .expect("Parse right half wrong");

                vec.push(left.to_string());
                vec.push(right.to_string());
                return vec;
            } else {
                let number = c.parse::<usize>().expect("Parse number wrong");
                vec.push((number * 2024).to_string());
            }
            vec
        });
    }
    println!("part1 {}", content.len());
}
