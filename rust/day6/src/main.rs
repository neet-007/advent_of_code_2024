use core::panic;
use std::{fs, usize};

fn main() {
    part1();
}

fn part2() {
    let mut content: Vec<Vec<String>> = fs::read_to_string("src/input.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut seen: Vec<Vec<bool>> = content.iter().map(|row| vec![false; row.len()]).collect();

    let mut pos: (isize, isize) = (0, 0);
    let mut dir = (-1, 0);

    'out: for (i, vec) in content.iter().enumerate() {
        for (j, c) in vec.iter().enumerate() {
            if c == "^" {
                pos = (i as isize, j as isize);
                seen[i][j] = true;
                break 'out;
            }
        }
    }

    loop {
        seen[pos.0 as usize][pos.1 as usize] = true;
        if pos.0 + dir.0 < 0
            || (pos.0 + dir.0) as usize >= content.len()
            || pos.1 + dir.1 < 0
            || (pos.1 + dir.1) as usize >= content[0].len()
        {
            break;
        }

        if content[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] == "#" {
            dir = rotate(dir);
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }

    let indices: Vec<(usize, usize)> = seen
        .iter()
        .enumerate()
        .flat_map(|(i, vec)| {
            vec.iter().enumerate().filter_map(
                move |(j, &val)| {
                    if val {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect();

    for check in indices.iter() {
        content[check.0][check.1] = String::from("#");

        search(&content, pos);

        content[check.0][check.1] = String::from(".");
    }
}

fn search(content: &Vec<Vec<String>>, mut pos: (isize, isize)) {
    let mut dir = (-1, 0);
    let mut seen: Vec<Vec<bool>> = content.iter().map(|row| vec![false; row.len()]).collect();
    loop {
        seen[pos.0 as usize][pos.1 as usize] = true;
        if pos.0 + dir.0 < 0
            || (pos.0 + dir.0) as usize >= content.len()
            || pos.1 + dir.1 < 0
            || (pos.1 + dir.1) as usize >= content[0].len()
        {
            break;
        }

        if content[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] == "#" {
            dir = rotate(dir);
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
}

fn part1() {
    let content: Vec<Vec<String>> = fs::read_to_string("src/input.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut seen: Vec<Vec<bool>> = content.iter().map(|row| vec![false; row.len()]).collect();

    let mut pos: (isize, isize) = (0, 0);
    let mut dir = (-1, 0);

    'out: for (i, vec) in content.iter().enumerate() {
        for (j, c) in vec.iter().enumerate() {
            if c == "^" {
                pos = (i as isize, j as isize);
                seen[i][j] = true;
                break 'out;
            }
        }
    }

    loop {
        seen[pos.0 as usize][pos.1 as usize] = true;
        if pos.0 + dir.0 < 0
            || (pos.0 + dir.0) as usize >= content.len()
            || pos.1 + dir.1 < 0
            || (pos.1 + dir.1) as usize >= content[0].len()
        {
            break;
        }

        if content[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] == "#" {
            dir = rotate(dir);
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }

    let sum = seen
        .iter()
        .map(|vec| vec.iter().filter(|&&val| val).count())
        .sum::<usize>();
    println!("part1: {}", sum);
}

fn rotate(pos: (isize, isize)) -> (isize, isize) {
    if pos.0 == 0 && pos.1 == 1 {
        return (1, 0);
    }
    if pos.0 == 0 && pos.1 == -1 {
        return (-1, 0);
    }
    if pos.0 == 1 && pos.1 == 0 {
        return (0, -1);
    }
    if pos.0 == -1 && pos.1 == 0 {
        return (0, 1);
    }

    panic!("invalid dir");
}
