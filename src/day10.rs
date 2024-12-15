use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    ops::Range,
};

type Map = Vec<Vec<i8>>;
type Bounds = (Range<i64>, Range<i64>);
type Point = (i64, i64);

fn load_map(path: &str) -> (Map, Bounds) {
    let mut map: Map = Vec::new();
    for line in read_to_string(path).unwrap().trim().lines() {
        // let mut row = Vec::new();
        // for c in line.chars() {
        //     row.push(match c.to_digit(10) {
        //         Some(i) => i.try_into().unwrap(),
        //         None => 20,
        //     });
        // }
        // map.push(row);
        map.push(
            line.chars()
                .map(|c| match c.to_digit(10) {
                    Some(i) => i.try_into().unwrap(),
                    None => 20,
                })
                .collect(),
        )
    }

    assert!(!map.is_empty());
    let width = map[0].len();
    for row in &map[1..] {
        assert_eq!(row.len(), width);
    }

    let bounds = ((0..map.len() as i64), (0..width as i64));
    return (map, bounds);
}

fn index_map(map: &Map, point: &Point) -> i8 {
    map[point.0 as usize][point.1 as usize]
}

fn in_bounds(bounds: &Bounds, point: &Point) -> bool {
    bounds.0.contains(&point.0) && bounds.1.contains(&point.1)
}

fn adjacent_points(point: &Point) -> [Point; 4] {
    [
        (point.0 - 1, point.1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1),
        (point.0, point.1 + 1),
    ]
}

pub fn part1() {
    let (map, bounds) = load_map("input/10/input.txt");

    let mut trail_heads = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 0 {
                trail_heads.push((i as i64, j as i64));
            }
        }
    }

    let mut sum = 0;

    for trail_head in trail_heads {
        // println!("{:?}", trail_head);

        let mut next_tiles = VecDeque::new();
        let mut visited_tiles = HashSet::new();

        next_tiles.push_back(trail_head);

        while let Some(current_tile) = next_tiles.pop_front() {
            // print!("\t{:?}", current_tile);

            if !visited_tiles.contains(&current_tile) {
                visited_tiles.insert(current_tile);

                let height = index_map(&map, &current_tile);
                // print!(" {}", height);

                match height {
                    0..=8 => {
                        adjacent_points(&current_tile)
                            .iter()
                            .filter(|&p| in_bounds(&bounds, p))
                            .filter(|&p| index_map(&map, p) == (height + 1))
                            .filter(|&p| !visited_tiles.contains(p))
                            .for_each(|&p| next_tiles.push_back(p));
                        // for next_tile in adjacent_4(&current_tile).iter()
                        // {
                        //     if in_bounds(&bounds, next_tile) {
                        //         if index_map(&map, next_tile) == (height + 1) {
                        //             if !visited_tiles.contains(next_tile) {
                        //                 next_tiles.push_back(*next_tile);
                        //             }
                        //         }
                        //     }
                        // }
                    }
                    9 => {
                        sum += 1;
                    }
                    _ => {}
                }
            }

            // println!();
        }
    }

    println!("day 10 part 1: {}", sum);
}

fn valid_next_points(map: &Map, bounds: &Bounds, height: i8, point: &Point) -> Vec<Point> {
    [
        (point.0 - 1, point.1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1),
        (point.0, point.1 + 1),
    ]
    .iter()
    .filter(|p| in_bounds(&bounds, *p))
    .filter(|p| index_map(&map, *p) == (height + 1))
    .cloned()
    .collect::<Vec<_>>()
}

fn depth_first_trail_search(map: &Map, bounds: &Bounds, current_position: &Point) -> i64 {
    let current_height = index_map(map, current_position);

    match current_height {
        0..=8 => valid_next_points(map, bounds, current_height, current_position)
            .iter()
            .map(|p| depth_first_trail_search(map, bounds, p))
            .sum(),
        9 => 1,
        _ => panic!(),
    }
}

pub fn part2() {
    let (map, bounds) = load_map("input/10/input.txt");

    let mut trail_heads = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 0 {
                trail_heads.push((i as i64, j as i64));
            }
        }
    }

    let sum: i64 = trail_heads
        .iter()
        .map(|p| depth_first_trail_search(&map, &bounds, p))
        .sum();

    println!("day 10 part 2: {}", sum);
}
