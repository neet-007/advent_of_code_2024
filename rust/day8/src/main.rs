use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn distance(p1: &(usize, usize), p2: &(usize, usize)) -> (isize, isize) {
    let dx = p2.1 as isize - p1.1 as isize;
    let dy = p2.0 as isize - p1.0 as isize;
    (dx, dy)
}

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("could not read file");

    let mut antennas: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut points: HashSet<(usize, usize)> = HashSet::new();

    let max_y = content.lines().count();
    let max_x = content.lines().next().map_or(0, |line| line.len());

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let key = c.to_string();
                antennas.entry(key).or_insert_with(Vec::new).push((y, x));
            }
        }
    }

    for (_, positions) in &antennas {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let (dx, dy) = distance(&p1, &p2);

                let and1 = (p1.0 as isize - dy, p1.1 as isize - dx);
                let and2 = (p2.0 as isize + dy, p2.1 as isize + dx);

                if and1.0 >= 0 && and1.1 >= 0 && and1.0 < max_y as isize && and1.1 < max_x as isize
                {
                    points.insert((and1.0 as usize, and1.1 as usize));
                }

                if and2.0 >= 0 && and2.1 >= 0 && and2.0 < max_y as isize && and2.1 < max_x as isize
                {
                    points.insert((and2.0 as usize, and2.1 as usize));
                }
            }
        }
    }

    println!("Unique points: {}", points.len());
}
