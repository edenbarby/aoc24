use std::{fs::read_to_string, slice::Iter};

pub fn seperate_string_into_numbers(string: &str, seperator: &str) -> Vec<i64> {
    string
        .trim()
        .split(seperator)
        .map_while(|t| t.parse::<i64>().ok())
        .collect()
}

fn load_input(path: &str) -> Vec<(i64, Vec<i64>)> {
    let mut input = Vec::new();
    for line in read_to_string(path).unwrap().trim().lines() {
        let tokens = line.split(':').collect::<Vec<_>>();
        assert!(tokens.len() == 2);
        let operands = seperate_string_into_numbers(tokens[1], " ");
        assert!(operands.len() > 1);
        input.push((tokens[0].parse::<i64>().unwrap(), operands));
    }
    input
}

fn solve_recursive_part1(resultant: i64, mut operands_iter: Iter<'_, i64>, total: i64) -> bool {
    if let Some(operand) = operands_iter.next() {
        return solve_recursive_part1(resultant, operands_iter.clone(), total + operand)
            || solve_recursive_part1(resultant, operands_iter.clone(), total * operand);
    }

    total == resultant
}

pub fn part1() {
    let inputs = load_input("input/07/input.txt");
    let mut sum = 0;
    for (resultant, operands) in inputs {
        if solve_recursive_part1(resultant, operands.iter(), 0) {
            sum += resultant;
        }
    }
    println!("day 7 part 1: {}", sum);
}

fn solve_recursive_part2(resultant: i64, mut operands_iter: Iter<'_, i64>, total: i64) -> bool {
    if let Some(operand) = operands_iter.next() {
        let concatenated = [total.to_string(), operand.to_string()]
            .concat()
            .parse::<i64>()
            .unwrap();

        return solve_recursive_part2(resultant, operands_iter.clone(), total + operand)
            || solve_recursive_part2(resultant, operands_iter.clone(), total * operand)
            || solve_recursive_part2(resultant, operands_iter.clone(), concatenated);
    }

    total == resultant
}

pub fn part2() {
    let inputs = load_input("input/07/input.txt");
    let mut sum = 0;
    for (resultant, operands) in inputs {
        if solve_recursive_part2(resultant, operands.iter(), 0) {
            sum += resultant;
        }
    }
    println!("day 7 part 1: {}", sum);
}
