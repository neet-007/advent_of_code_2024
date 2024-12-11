use std::{collections::HashSet, fs, usize};

fn walk(mut y: usize, mut x: usize, matrix: &Vec<Vec<String>>) {
    let mut set = HashSet::new();
    let mut prev = 0;
    let mut curr = 0;

    loop {
        if x != 0
            && !set.contains(&(y, x - 1))
            && matrix[y][x - 1] != "."
            && matrix[y][x - 1].parse::<usize>().expect("must parse num") == curr + 1
        {
            x -= 1;
            set.insert((y, x - 1));
            prev = curr;
            curr += 1;
        } else if x + 1 < matrix[y].len()
            && !set.contains(&(y, x + 1))
            && matrix[y][x + 1] != "."
            && matrix[y][x + 1].parse::<usize>().expect("must parse num") == curr + 1
        {
            x += 1;
            set.insert((y, x + 1));
            prev = curr;
            curr += 1;
        } else if y != 0
            && !set.contains(&(y - 1, x))
            && matrix[y - 1][x] != "."
            && matrix[y - 1][x].parse::<usize>().expect("must parse num") == curr + 1
        {
            y -= 1;
            set.insert((y - 1, x));
            prev = curr;
            curr += 1;
        } else if y + 1 < matrix.len()
            && !set.contains(&(y + 1, x))
            && matrix[y + 1][x] != "."
            && matrix[y + 1][x].parse::<usize>().expect("must parse num") == curr + 1
        {
            y += 1;
            set.insert((y + 1, x));
            prev = curr;
            curr += 1;
        }

        if prev == curr {
            break;
        }
    }
}

fn main() {
    let content = fs::read_to_string("src/input.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
}
