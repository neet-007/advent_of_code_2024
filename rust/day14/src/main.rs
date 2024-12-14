use std::{fs, isize};

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn main() {
    let mut content = fs::read_to_string("src/input.txt")
        .expect("could not read file")
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").expect("no space");
            let p_coords = p
                .split("=")
                .skip(1)
                .next()
                .expect("x, y not found")
                .split(",")
                .map(|c| c.parse::<isize>().expect("could not parse num"))
                .collect::<Vec<isize>>();

            let v_coords = v
                .split("=")
                .skip(1)
                .next()
                .expect("x, y not found")
                .split(",")
                .map(|c| c.parse::<isize>().expect("could not parse num"))
                .collect::<Vec<isize>>();

            ((p_coords[0], p_coords[1]), (v_coords[0], v_coords[1]))
        })
        .collect::<Vec<((isize, isize), (isize, isize))>>();

    let x_half = WIDTH / 2;
    let y_half = HEIGHT / 2;
    for _ in 0..100 {
        for i in 0..content.len() {
            let ((px, py), (vx, vy)) = content[i];
            let mut new_x = px + vx;
            if new_x >= WIDTH {
                new_x = new_x % WIDTH;
            }
            if new_x < 0 {
                new_x = WIDTH + new_x;
            }
            let mut new_y = py + vy;
            if new_y >= HEIGHT {
                new_y = new_y % HEIGHT;
            }
            if new_y < 0 {
                new_y = HEIGHT + new_y;
            }
            content[i] = ((new_x, new_y), (vx, vy));
        }
    }

    let res = content.iter().fold((0, 0, 0, 0), |g, c| {
        let (p, _) = c;
        if p.0 > x_half && p.1 > y_half {
            return (g.0 + 1, g.1, g.2, g.3);
        }
        if p.0 > x_half && p.1 < y_half {
            return (g.0, g.1 + 1, g.2, g.3);
        }
        if p.0 < x_half && p.1 > y_half {
            return (g.0, g.1, g.2 + 1, g.3);
        }
        if p.0 < x_half && p.1 < y_half {
            return (g.0, g.1, g.2, g.3 + 1);
        }
        g
    });

    println!("part1: {}", res.0 * res.1 * res.2 * res.3);
}
