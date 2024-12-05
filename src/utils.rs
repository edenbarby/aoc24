use std::{fs::read_to_string, str::Lines};

pub fn extract_ints_from_string(s: &str) -> Vec<i32> {
    s.split(" ")
        .skip_while(|t| t.len() == 0)
        .map(|t| i32::from_str_radix(t, 10).unwrap())
        .collect()
}

pub fn load_pair_of_columns(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut input_left = Vec::new();
    let mut input_right = Vec::new();

    for line in read_to_string(filename).unwrap().trim().lines() {
        let mut i = 0;
        for token in line.split(" ") {
            if token.len() > 0 {
                let num = i32::from_str_radix(token, 10).unwrap();
                if i == 0 {
                    input_left.push(num);
                } else {
                    input_right.push(num);
                }
                i += 1;
            }
            if i == 2 {
                break;
            }
        }
    }

    (input_left, input_right)
}

pub fn seperate_string_into_numbers(string: &str, seperator: &str) -> Vec<i32> {
    string
        .trim()
        .split(seperator)
        .map_while(|t| t.parse::<i32>().ok())
        .collect()
}
