use std::{fs, usize};

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("Could not read file");

    let parsed_content: Vec<Vec<(usize, usize)>> = content
        .split("\n\n")
        .map(|seg| {
            seg.lines()
                .enumerate()
                .map(|(i, line)| {
                    // Split the line at the colon
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() < 2 {
                        panic!("Line must contain a colon: {}", line);
                    }

                    let (l, r) = parts[1]
                        .split_once(',')
                        .expect("Part after colon must contain a comma");

                    let parse_number = |input: &str, separator: char| -> usize {
                        input
                            .split(separator)
                            .skip(1)
                            .next()
                            .expect("Must have number after separator")
                            .parse::<usize>()
                            .expect("Must parse number")
                    };

                    let l_num = if i == 2 {
                        parse_number(l, '=')
                    } else {
                        parse_number(l, '+')
                    };

                    let r_num = if i == 2 {
                        parse_number(r, '=')
                    } else {
                        parse_number(r, '+')
                    };

                    (l_num, r_num)
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let sum = parsed_content
        .iter()
        .map(|part| {
            let mut part_iter = part.iter();
            let a = part_iter.next().expect("must have a");
            let b = part_iter.next().expect("must have b");
            let t = part_iter.next().expect("must have t");

            let mut count_a = 0;
            let mut count_b = 100;

            let mut found = (a.0 * count_a + b.0 * count_b, a.1 * count_a + b.1 * count_b);

            while found.0 != t.0 && found.1 != t.1 && count_a != 100 {
                count_a += 1;
                count_b -= 1;

                found = (a.0 * count_a + b.0 * count_b, a.1 * count_a + b.1 * count_b);
                println!("{count_a} {count_b} {:?}", found);
            }

            if count_a == 100 {
                (0, 0)
            } else {
                (count_a, count_b)
            }
        })
        .fold(0, |sum, next| sum + next.0 * 3 + next.1);

    println!("{sum}");
}
