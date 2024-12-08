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
        let safe = (all_increasing || all_decreasing) && (1..=3).contains(&md);

        if safe {
            sum += 1;
        }
    }

    println!("day2_part1: {}", sum);
}

fn is_safe(readings: &[i32]) -> bool {
    if readings.is_empty() {
        return true;
    }

    let mut last_reading = readings[0];
    let mut increasing = true;
    let mut decreasing = true;
    let mut diff_min = u32::MAX;
    let mut diff_max = 0;

    for &reading in &readings[1..] {
        println!("{} -> {}", last_reading, reading);
        if reading <= last_reading {
            increasing = false;
            println!("not increasing")
        }
        if reading >= last_reading {
            decreasing = false;
            println!("not decreasing")
        }
        let diff = reading.abs_diff(last_reading);
        println!("diff {}", diff);
        if diff < diff_min {
            diff_min = diff;
        }
        if diff > diff_max {
            diff_max = diff;
        }
        last_reading = reading;
    }

    println!(
        "(increasing:{} || decreasing:{}) && (diff_min:{} >= 1) && (diff_max:{} <= 3)",
        increasing, decreasing, diff_min, diff_max,
    );
    (increasing || decreasing) && (diff_min >= 1) && (diff_max <= 3)
}

pub fn day2_part2() {
    let filename = "input/02/input_1.txt";

    let mut sum = 0;
    for line in read_to_string(filename).unwrap().trim().lines() {
        let readings = extract_ints_from_string(line);

        let mut safe = is_safe(&readings);
        println!("{:?} {}", readings, safe);

        if !safe {
            for n in 0..readings.len() {
                let before = &readings[..n];
                let after = &readings[n + 1..];
                let new_readings = [before, after].concat();

                safe = is_safe(&new_readings);
                println!("{:?} {}", new_readings, safe);
                if safe {
                    break;
                }
            }
        }

        if safe {
            sum += 1;
        }

        // println!("{}", safe);
    }

    println!("day2_part2: {}", sum);
}
