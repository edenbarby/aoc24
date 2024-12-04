use crate::utils::load_pair_of_columns;
use std::{collections::HashMap, iter::zip};

pub fn day1_part1() {
    let (mut input_left, mut input_right) = load_pair_of_columns("input/01/input_1.txt");

    input_left.sort();
    input_right.sort();

    let mut sum = 0;
    for (l, r) in zip(input_left, input_right) {
        sum += l.abs_diff(r);
    }

    println!("day 1 part 1: {}", sum);
}

pub fn day1_part2() {
    let (input_left, input_right) = load_pair_of_columns("input/01/input_1.txt");

    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for x in input_right {
        let y_new = match count_map.get(&x) {
            Some(&y) => y + 1,
            None => 1,
        };
        count_map.insert(x, y_new);
    }

    let mut sum = 0;
    for x in input_left {
        let count = match count_map.get(&x) {
            Some(&y) => y,
            None => 0,
        };
        sum += x * count;
    }

    println!("day 1 part 2: {}", sum);
}
