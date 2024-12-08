use std::fs::read_to_string;

use regex::Regex;

pub fn part1() {
    let filename = "input/03/input.txt";

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum = 0;

    let input = read_to_string(filename).unwrap();

    for (_, [x_str, y_str]) in re.captures_iter(&input).map(|c| c.extract()) {
        let x: i32 = x_str.parse().unwrap();
        let y: i32 = y_str.parse().unwrap();

        sum += x * y;
    }

    println!("day 3 part 1: {}", sum);
}

pub fn part2() {
    let filename = "input/03/input.txt";

    let instr_regex = Regex::new(r"mul\([,0-9]+\)|do\(\)|don\'t\(\)").unwrap();
    let mul_arg_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let input = read_to_string(filename).unwrap();

    let mut mul_enabled = true;
    let mut sum = 0;

    for instruction in instr_regex.find_iter(&input).map(|f| f.as_str()) {
        if instruction == "do()" {
            mul_enabled = true;
        } else if instruction == "don't()" {
            mul_enabled = false;
        } else if mul_enabled && (&instruction[..4] == "mul(") {
            let (_, [x_str, y_str]) = mul_arg_regex.captures(instruction).unwrap().extract();

            let x: i32 = x_str.parse().unwrap();
            let y: i32 = y_str.parse().unwrap();

            sum += x * y;
        }
    }

    println!("day 3 part 1: {}", sum);
}
