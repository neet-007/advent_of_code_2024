use std::fs;

#[derive(Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Op {
    op_code: OpCode,
    operand: usize,
}

fn main() {
    let mut registers = vec![0, 0, 0];
    let mut content = fs::read_to_string("src/input.txt")
        .expect("could not read file")
        .split("\n\n")
        .map(|seg| seg.to_owned())
        .collect::<Vec<String>>()
        .into_iter();

    content
        .next()
        .expect("must have first section")
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            registers[i] = line
                .split_once(":")
                .expect("must have colon")
                .1
                .trim()
                .parse::<usize>()
                .expect("must parse number");
        });

    let operations: Vec<Op> = content
        .next()
        .expect("must have second section")
        .split_once(":")
        .expect("must have colon")
        .1
        .trim()
        .split(",")
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|pair| {
            let op_code = match pair[0] {
                "0" => OpCode::Adv,
                "1" => OpCode::Bxl,
                "2" => OpCode::Bst,
                "3" => OpCode::Jnz,
                "4" => OpCode::Bxc,
                "5" => OpCode::Out,
                "6" => OpCode::Bdv,
                "7" => OpCode::Cdv,
                _ => panic!("Invalid opcode"),
            };
            let operand = pair[1].parse::<usize>().expect("must parse operand");
            Op { op_code, operand }
        })
        .collect();

    let mut out: Vec<usize> = vec![];

    let compos = |operand: usize, registers: &Vec<usize>| -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            7 => panic!("invalid program"),
            _ => panic!("invalid combo"),
        }
    };

    let mut i = 0;
    while i < operations.len() {
        let op = &operations[i];
        match op.op_code {
            OpCode::Adv => {
                let compo = compos(op.operand, &registers);
                registers[0] = registers[0] / (2_usize.pow(compo as u32));
            }
            OpCode::Bxl => {
                registers[1] = registers[1] ^ op.operand;
            }
            OpCode::Bst => {
                let compo = compos(op.operand, &registers);
                registers[1] = compo % 8;
            }
            OpCode::Jnz => {
                if registers[0] != 0 {
                    i = op.operand;
                    continue;
                }
            }
            OpCode::Bxc => {
                registers[1] = registers[1] ^ registers[2];
            }
            OpCode::Out => {
                let compo = compos(op.operand, &registers);
                out.push(compo % 8);
            }
            OpCode::Bdv => {
                let compo = compos(op.operand, &registers);
                registers[1] = registers[0] / (2_usize.pow(compo as u32));
            }
            OpCode::Cdv => {
                let compo = compos(op.operand, &registers);
                registers[2] = registers[0] / (2_usize.pow(compo as u32));
            }
        };
        i += 1;
    }

    println!(
        "{}",
        out.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
