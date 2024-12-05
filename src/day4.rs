use std::{collections::HashSet, fs::read_to_string, io};

pub fn part1() {
    let filename = "input/04/input_1.txt";
    //let filename = "input/04/test.txt";
    let mut board: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(filename).unwrap().trim().lines() {
        board.push(line.chars().collect());
    }

    assert!(board.len() > 0);
    let board_width = board[0].len();
    assert!(board.iter().all(|l| l.len() == board_width));

    let word: Vec<char> = "XMAS".chars().collect();

    let idx_steps = [
        (0isize, 1isize), // East
        (1, 1),           // Southeast
        (1, 0),           // South
        (1, -1),          // Southwest
        (0, -1),          // West
        (-1, -1),         // Northwest
        (-1, 0),          // North
        (-1, 1),          // Northeast
    ];

    let mut matches = 0;

    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == word[0] {
                for (row_idx_step, col_idx_step) in idx_steps {
                    let mut match_failed = false;

                    for n in 1..word.len() {
                        let n_signed: isize = n.try_into().unwrap();
                        let i_opt = row_idx.checked_add_signed(n_signed * row_idx_step);
                        let j_opt = col_idx.checked_add_signed(n_signed * col_idx_step);

                        if i_opt.is_some() && j_opt.is_some() {
                            let i = i_opt.unwrap();
                            let j = j_opt.unwrap();
                            if i < board.len() && j < board_width {
                                if board[i][j] != word[n] {
                                    match_failed = true;
                                    break;
                                }
                            } else {
                                // Gone off the right or bottom side of the board.
                                match_failed = true;
                                break;
                            }
                        } else {
                            // Gone off the top or left side of the board.
                            // Indexes are negative (or less likely overflowed the int).
                            match_failed = true;
                            break;
                        }
                    }

                    if !match_failed {
                        matches += 1;
                    }
                }
            }
        }
    }

    println!("day 4 part 1: {}", matches);
}

pub fn part2() {
    let filename = "input/04/input_1.txt";
    // let filename = "input/04/test.txt";
    let mut board: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(filename).unwrap().trim().lines() {
        board.push(line.chars().collect());
    }

    assert!(board.len() > 0);
    let board_width = board[0].len();
    assert!(board.iter().all(|l| l.len() == board_width));

    let mut matches = 0;

    let read = |i_opt: Option<usize>, j_opt: Option<usize>| -> Option<char> {
        if i_opt.is_some() && j_opt.is_some() {
            let i = i_opt.unwrap();
            let j = j_opt.unwrap();

            if i < board.len() && j < board_width {
                return Some(board[i][j]);
            }
        }
        None
    };

    let idx_steps = [
        (1isize, 1isize), // Southeast
        (-1, -1),         // Northwest
        (1, -1),          // Southwast
        (-1, 1),          // Northeast
    ];

    let expected_pair = HashSet::from(['M', 'S']);

    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == 'A' {
                let neighbours: Vec<char> = idx_steps
                    .iter()
                    .map_while(|(i, j)| {
                        read(
                            row_idx.checked_add_signed(*i),
                            col_idx.checked_add_signed(*j),
                        )
                    })
                    .collect();

                if neighbours.len() == 4 {
                    let mut both_pairs_present = true;

                    for pair in neighbours.chunks(2) {
                        let pair_set: HashSet<char> = pair.iter().map(|&x| x).collect();

                        if pair_set != expected_pair {
                            both_pairs_present = false;
                            break;
                        }
                    }

                    if both_pairs_present {
                        matches += 1;
                    }
                }
            }
        }
    }

    println!("day 4 part 1: {}", matches);
}
