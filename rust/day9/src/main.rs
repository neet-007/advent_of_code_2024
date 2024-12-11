use std::fs;

fn main() {
    part1();
    part2();
}

fn part2() {
    let content = fs::read_to_string("src/input.txt").expect("could not read file");
    let mut result = content
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().expect("could not parse num"))
        .enumerate()
        .fold((Vec::<isize>::new(), -1), |(mut vec, last), (i, num)| {
            let mut new_last = last;
            if i % 2 == 0 {
                new_last += 1;
                for _ in 0..num {
                    vec.push(new_last);
                }
            } else {
                for _ in 0..num {
                    vec.push(-1);
                }
            }
            (vec, new_last)
        })
        .0;

    let mut last = result.len() - 1;
    let mut last_end = last;
    let mut i = 0;
    let mut i_end = i;
    let mut comp = false;

    while i < last {
        if !comp && result[i] != -1 {
            i += 1;
            i_end + 1;
            continue;
        }

        comp = true;
        while result[i_end] == -1 {
            i_end += 1;
        }

        while result[last_end] == result[last] {
            last_end -= 1;
        }

        if last_end.abs_diff(last) > i_end.abs_diff(i) {
            last = last_end - 1;
            continue;
        }

        result.swap(i, last);

        last -= 1;
    }

    let ans: usize = result[0..=last]
        .iter()
        .enumerate()
        .map(|(i, c)| i * *c as usize)
        .sum();

    println!("part1: {}", ans);
}

fn part1() {
    let content = fs::read_to_string("src/input.txt").expect("could not read file");
    let mut result = content
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().expect("could not parse num"))
        .enumerate()
        .fold((Vec::<isize>::new(), -1), |(mut vec, last), (i, num)| {
            let mut new_last = last;
            if i % 2 == 0 {
                new_last += 1;
                for _ in 0..num {
                    vec.push(new_last);
                }
            } else {
                for _ in 0..num {
                    vec.push(-1);
                }
            }
            (vec, new_last)
        })
        .0;

    let mut last = result.len() - 1;

    for i in 0..result.len() {
        if result[i] != -1 {
            continue;
        }

        while result[last] == -1 {
            last -= 1;
        }

        if i >= last {
            break;
        }
        result.swap(i, last);

        last -= 1;
    }

    let ans: usize = result[0..=last]
        .iter()
        .enumerate()
        .map(|(i, c)| i * *c as usize)
        .sum();

    println!("part1: {}", ans);
}
