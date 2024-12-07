use std::fs;

fn can_calibrate(operands: &[usize], target: usize, index: usize, current_value: usize) -> bool {
    if index == operands.len() {
        return current_value == target;
    }

    if can_calibrate(operands, target, index + 1, current_value + operands[index]) {
        return true;
    }

    if can_calibrate(operands, target, index + 1, current_value * operands[index]) {
        return true;
    }

    if let Ok(concatenated_value) = format!("{}{}", current_value, operands[index]).parse::<usize>()
    {
        if can_calibrate(operands, target, index + 1, concatenated_value) {
            return true;
        }
    }

    false
}

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("could not read file");

    let mut total_calibration_result = 0;

    for line in content.lines() {
        if let Some((val, operands)) = line.split_once(':') {
            let target_value: usize = val.trim().parse().expect("Invalid target value");

            let operands_list: Vec<usize> = operands
                .split_whitespace()
                .map(|num| num.parse::<usize>().expect("Invalid operand"))
                .collect();

            if can_calibrate(&operands_list, target_value, 1, operands_list[0]) {
                total_calibration_result += target_value;
            }
        }
    }

    println!("Total Calibration Result: {}", total_calibration_result);
}
