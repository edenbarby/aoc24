use crate::utils::*;
use std::{fs::read_to_string, iter::zip};

pub fn day2_part1() {
    let filename = "input/02/input_1.txt";

    let mut sum = 0;
    for line in read_to_string(filename).unwrap().trim().lines() {
        let nums = extract_ints_from_string(line);

        let mut all_increasing = true;
        let mut all_decreasing = true;
        let mut max_diff = None;

        for (&x0, &x1) in zip(&nums[..nums.len() - 1], &nums[1..]) {
            let increasing = x1 > x0;
            let decreasing = x1 < x0;
            let diff = x0.abs_diff(x1);

            if !increasing {
                all_increasing = false;
            }
            if !decreasing {
                all_decreasing = false;
            }

            max_diff = match max_diff {
                Some(md) => {
                    if diff > md {
                        Some(diff)
                    } else {
                        Some(md)
                    }
                }
                None => Some(diff),
            }
        }

        let md = max_diff.unwrap();
        let safe = (all_increasing || all_decreasing) && (md >= 1) && (md <= 3);

        if safe {
            sum += 1;
        }
    }

    println!("day2_part1: {}", sum);
}

// fn is_safe(readings: Vec<i32>) -> bool {
//     if readings.len() > 0 {
//         let last_reading = readings[0];
//         let mut increasing = true;
//         let mut decreasing = true;
//         let mut diff_min = u32
//         for reading in readings {
//             if !(reading > last_reading) {

//             }
//         }
//     }
// }

pub fn day2_part2() {
    let filename = "input/02/input_1.txt";

    let mut sum = 0;
    for line in read_to_string(filename).unwrap().trim().lines() {
        let nums = extract_ints_from_string(line);

        let mut all_increasing = true;
        let mut all_decreasing = true;
        let mut min_diff = u32::MAX;
        let mut max_diff = 0;
        let mut safe = true;
        let mut already_skipped = false;

        let mut diffs = vec![];

        for (&x0, &x1) in zip(&nums[..nums.len() - 1], &nums[1..]) {
            let increasing = x1 > x0;
            let decreasing = x1 < x0;
            let diff = x0.abs_diff(x1);

            if !increasing {
                all_increasing = false;
            }
            if !decreasing {
                all_decreasing = false;
            }

            if diff < min_diff {
                min_diff = diff;
            }
            if diff > max_diff {
                max_diff = diff;
            }

            let new_safe = (all_increasing || all_decreasing) && (min_diff >= 1) && (max_diff <= 3);
            if new_safe != safe {
                if !already_skipped {
                    already_skipped = true;
                } else {
                    safe = new_safe;
                }
            }

            diffs.push(x1 - x0);
            // print!("{} ", x1 - x0);
        }

        if safe {
            sum += 1;
        }

        if !safe {
            for n in diffs {
                print!("{} ", n)
            }
            println!("safe {} skipped {}", safe, already_skipped);
        }
    }

    println!("day2_part2: {}", sum);
}
