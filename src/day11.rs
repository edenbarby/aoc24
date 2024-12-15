use std::{fs::read_to_string, iter, thread};

type Stones = Vec<u64>;

fn load_stones(path: &str) -> Stones {
    let stones = read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map_while(|t| t.parse::<u64>().ok())
        .collect::<Stones>();
    assert!(!stones.is_empty());
    stones
}

fn display_stones(stones: &Stones) {
    for s in stones {
        print!("{} ", s);
    }
    println!();
}

pub fn part1() {
    let mut stones = load_stones("input/11/input.txt");
    // display_stones(&stones);

    for _i in 0..25 {
        let mut new_stones = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let digits = stone.to_string();
                if digits.len() % 2 == 0 {
                    let mid = digits.len() / 2;
                    let left_stone = digits[..mid].parse::<u64>().unwrap();
                    let right_stone = digits[mid..].parse::<u64>().unwrap();

                    new_stones.push(left_stone);
                    new_stones.push(right_stone);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }
        stones = new_stones;

        //display_stones(&stones);
        // println!("{} {}", i + 1, stones.len());
    }

    println!("day 11 part 1: {}", stones.len());
}

// fn count_digits(mut x: u64) -> u64 {
//     let mut digit_count = 0;
//     loop {
//         match x {
//             0..10 => return digit_count + 1,
//             10..100 => return digit_count + 2,
//             100..1000 => return digit_count + 3,
//             1000..10000 => return digit_count + 4,
//             10000..100000 => return digit_count + 5,
//             100000..1000000 => return digit_count + 6,
//             1000000..10000000 => return digit_count + 7,
//             _ => {
//                 x /= 10000000;
//                 digit_count += 7;
//             }
//         }
//     }
// }

fn count_digits(x: u64) -> u64 {
    match x {
        0..10 => 1,
        10..100 => 2,
        100..1000 => 3,
        1000..10000 => 4,
        10000..100000 => 5,
        100000..1000000 => 6,
        1000000..10000000 => 7,
        _ => (x as f32).log10().ceil() as u64,
    }
}

// fn split_digits(x: u64, count: u64) -> Vec<u64> {
//     // assert!(count % 2 == 0);
//     let shift = 10u64.pow((count / 2) as u32);
//     let left = x / shift;
//     vec![left, x - (left * shift)]
// }

fn split_digits(x: u64, count: u64, stones: &mut Vec<u64>) {
    let shift = 10u64.pow((count / 2) as u32);
    let left = x / shift;
    stones.push(left);
    stones.push(x - (left * shift));
}

// fn mutate_stone(x: u64) -> Vec<u64> {
//     if x == 0 {
//         vec![1]
//     } else {
//         let digits = count_digits(x);
//         if digits % 2 == 0 {
//             split_digits(x, digits)
//         } else {
//             vec![x * 2024]
//         }
//     }
// }

fn mutate_stone(x: u64, stones: &mut Vec<u64>) {
    if x == 0 {
        stones.push(x);
    } else {
        let digits = count_digits(x);
        if digits % 2 == 0 {
            split_digits(x, digits, stones);
        } else {
            stones.push(x * 2024);
        }
    }
}

// fn mutate_stones(stones: &Stones) -> Stones {
//     stones
//         .iter()
//         .flat_map(|&s| mutate_stone(s))
//         .collect::<Stones>()
// }

fn mutate_stones(stones: &Stones) -> Stones {
    let mut new_stones = Vec::with_capacity(stones.len());

    for &stone in stones {
        mutate_stone(stone, &mut new_stones);
    }

    new_stones
}

// fn count_stones(mut stones: Stones, iterations: usize, _worker_id: usize) -> usize {
//     for _i in 0..iterations {
//         stones = mutate_stones(&stones);

//         println!(
//             "{:03} -> {:02}/{:02} {}",
//             _worker_id,
//             _i + 1,
//             iterations,
//             stones.len()
//         );
//     }

//     stones.len()
// }

fn count_stones(mut stones: Stones, iterations: usize, _worker_id: usize) -> usize {
    for _i in 0..iterations {
        stones = mutate_stones(&stones);

        println!(
            "{:03} -> {:02}/{:02} {}",
            _worker_id,
            _i + 1,
            iterations,
            stones.len()
        );
    }

    let mut max = 0;
    for &s in stones.iter() {
        if s > max {
            max = s;
        }
    }
    println!("max: {}", max);

    stones.len()
}

pub fn part2() {
    assert_eq!(count_digits(0), 1);
    assert_eq!(count_digits(1), 1);
    assert_eq!(count_digits(13), 2);
    assert_eq!(count_digits(331), 3);
    assert_eq!(count_digits(3310), 4);
    assert_eq!(count_digits(3200230310), 10);
    assert_eq!(count_digits(320000011230310), 15);
    assert_eq!(count_digits(300000112331652310), 18);

    // assert_eq!(split_digits(13, 2), vec![1, 3]);
    // assert_eq!(split_digits(3310, 4), vec![33, 10]);
    // assert_eq!(split_digits(3200230310, 10), vec![32002, 30310]);
    // assert_eq!(
    //     split_digits(300000112331652310, 18),
    //     vec![300000112, 331652310]
    // );

    // assert_eq!(mutate_stone(0), vec![1]);
    // assert_eq!(mutate_stone(1), vec![2024]);
    // assert_eq!(mutate_stone(13), vec![1, 3]);
    // assert_eq!(mutate_stone(331), vec![331 * 2024]);
    // assert_eq!(mutate_stone(3310), vec![33, 10]);
    // assert_eq!(mutate_stone(3200230310), vec![32002, 30310]);
    // assert_eq!(mutate_stone(320000011230310), vec![320000011230310 * 2024]);
    // assert_eq!(mutate_stone(300000112331652310), vec![300000112, 331652310]);

    let iterations = 40;
    let stones = load_stones("input/11/input.txt");

    println!("day 11 part 2: {}", count_stones(stones, iterations, 0));

    // let mut iterations = 75;
    // let mut stones = load_stones("input/11/input.txt");

    // let thread_count = 4;
    // while stones.len() < 3 * thread_count {
    //     iterations -= 1;
    //     stones = mutate_stones(&stones);
    // }
    // let stones_per_worker = stones.len() / thread_count;

    // let worker_handles = stones
    //     .chunks(stones_per_worker)
    //     .enumerate()
    //     .map(|(i, s)| {
    //         // println!("{:03} -> starting with {} stones!", i, s.len());
    //         let v = s.to_vec();
    //         thread::spawn(move || count_stones_2(v, iterations, i))
    //     })
    //     .collect::<Vec<_>>();

    // println!(
    //     "day 11 part 2: {}",
    //     worker_handles
    //         .into_iter()
    //         .map(|w| w.join().unwrap())
    //         .sum::<usize>()
    // );
}
