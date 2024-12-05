use std::cmp::Ordering;
use std::{collections::HashMap, fs, usize};

fn main() {
    part1();
    part2();
}

fn part2() {
    // found this on reddit leanr from it
    let input = fs::read_to_string("src/input.txt").unwrap();
    let rules = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum: u32 = updates
        .into_iter()
        .filter(|pages| !pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a))))
        .map(|mut update| {
            update.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Greater
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|pages| pages[pages.len() / 2])
        .sum();

    println!("part2 {}", sum);
}

fn part1() {
    let content = fs::read_to_string("src/input.txt").expect("could not read file");
    let section: Vec<&str> = content.split("\n\n").collect();

    let mut rules = HashMap::<&str, Vec<&str>>::new();

    for line in section[0].lines() {
        let parts: Vec<&str> = line.split("|").collect();

        rules
            .entry(parts[1])
            .or_insert_with(Vec::new)
            .push(parts[0]);
    }

    let mut sum = 0;

    'lines: for line in section[1].lines() {
        let nums: Vec<&str> = line.split(',').collect();

        for i in 0..nums.len() {
            if let Some(vec) = rules.get(&nums[i]) {
                for j in i + 1..nums.len() {
                    if vec.contains(&nums[j]) {
                        continue 'lines;
                    }
                }
            }
        }

        let mid: usize = nums[nums.len() / 2].parse().expect("not a number");
        sum += mid;
    }

    println!("part1: {}", sum);
}
