use std::{fs, isize, usize};

fn main() {
    let mut content = fs::read_to_string("src/input.txt")
        .expect("could not read file")
        .split("\n\n")
        .map(|str| str.to_owned())
        .collect::<Vec<_>>()
        .into_iter();

    let mut pos = (0, 0);
    let mut map = content
        .next()
        .expect("must have first")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '@' {
                        pos = (y, x);
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let moves = content
        .next()
        .expect("must have last")
        .trim()
        .chars()
        .collect::<Vec<_>>();

    for m in moves {
        let (x, y, flag) = check_move(m, pos.0, pos.1, &mut map);
        if flag == -1 {
            continue;
        }

        pos = (x, y);
    }

    for row in map {
        println!("{row:?}");
    }
}

fn check_move(
    m: char,
    mut x: usize,
    mut y: usize,
    map: &mut Vec<Vec<char>>,
) -> (usize, usize, isize) {
    if m == '^' {
        let mut count = 0;
        while map[y][x] != '#' && map[y][x] != '.' {
            count += 1;
            y -= 1;
        }
        if map[y][x] == '#' {
            return (0, 0, -1);
        }

        for _ in 0..=count {
            let temp = map[y][x];
            map[y][x] = map[y + 1][x];
            map[y + 1][x] = temp;
            y += 1;
        }
        return (x, y - 1, 0);
    }
    if m == 'v' {
        let mut count = 0;
        while map[y][x] != '#' && map[y][x] != '.' {
            count += 1;
            y += 1;
        }
        if map[y][x] == '#' {
            return (0, 0, -1);
        }

        for _ in 0..=count {
            let temp = map[y][x];
            map[y][x] = map[y - 1][x];
            map[y - 1][x] = temp;
            y -= 1;
        }
        return (x, y + 1, 0);
    }
    if m == '>' {
        let mut count = 0;
        while map[y][x] != '#' && map[y][x] != '.' {
            count += 1;
            x += 1;
        }
        if map[y][x] == '#' {
            return (0, 0, -1);
        }

        for _ in 0..=count {
            let temp = map[y][x];
            map[y][x] = map[y][x + 1];
            map[y][x + 1] = temp;
            x -= 1;
        }
        return (x + 1, y, 0);
    }
    if m == '<' {
        let mut count = 0;
        while map[y][x] != '#' && map[y][x] != '.' {
            count += 1;
            x -= 1;
        }
        if map[y][x] == '#' {
            return (0, 0, -1);
        }

        for _ in 0..=count {
            let temp = map[y][x];
            map[y][x] = map[y][x - 1];
            map[y][x - 1] = temp;
            x += 1;
        }
        return (x - 1, y, 0);
    }

    panic!("invalid move");
}
