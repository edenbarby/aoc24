use std::{collections::HashSet, ops::Range, vec};

use crate::utils::load_2d_array;

type Map = Vec<Vec<char>>;
type Position = (i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_direction(c: &char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::Up),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        _ => None,
    }
}

fn get_next_direction(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_vector(d: Direction) -> (i32, i32) {
    match d {
        Direction::Up => (-1, 0),
        Direction::Right => (0, 1),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
    }
}

fn move_pos(pos: &(i32, i32), vec: &(i32, i32)) -> (i32, i32) {
    (pos.0 + vec.0, pos.1 + vec.1)
}

fn in_bounds(pos: &(i32, i32), bounds: &(Range<i32>, Range<i32>)) -> bool {
    bounds.0.contains(&pos.0) && bounds.1.contains(&pos.1)
}

fn display_board(map: &Map, trace: &[(i32, i32)]) {
    let mut display = map.clone();

    for &(row, col) in trace {
        display[row as usize][col as usize] = 'X';
    }

    for row in &display {
        for c in row {
            print!("{}", c)
        }
        println!();
    }
}

fn get_started(map: &Map) -> Option<(Position, Direction)> {
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if let Some(dir) = get_direction(c) {
                return Some(((i as i32, j as i32), dir));
            }
        }
    }
    None
}

pub fn part1() {
    let input_file_path = "input/06/input.txt";
    let (map, map_width) = load_2d_array(input_file_path);
    let bounds = ((0..map.len() as i32), (0..map_width as i32));

    let (mut position, mut direction) = get_started(&map).unwrap();

    let mut done = false;
    let mut trace = vec![position];
    let mut i = 0;
    while !done {
        i += 1;
        let vector = get_vector(direction);
        let new_position = move_pos(&position, &vector);

        if in_bounds(&new_position, &bounds) {
            if map[new_position.0 as usize][new_position.1 as usize] == '#' {
                direction = get_next_direction(direction);
            } else {
                position = new_position;
                trace.push(position);
            }
        } else {
            done = true;
        }
    }

    display_board(&map, &trace);

    let deduplicated = trace.iter().collect::<HashSet<_>>();
    println!("i = {}", i);
    println!("day 6 part 1: {}", deduplicated.len());
}

type MapBounds = (Range<i32>, Range<i32>);
fn load_map(path: &str) -> (Map, MapBounds) {
    let (map, map_width) = load_2d_array(path);
    let bounds = ((0..map.len() as i32), (0..map_width as i32));
    (map, bounds)
}

fn is_obstacle(map: &Map, position: &Position) -> bool {
    let i = position.0 as usize;
    let j = position.1 as usize;
    let tile = map[i][j];
    tile == '#' || tile == 'O'
}

fn walk(
    map: &Map,
    bounds: &MapBounds,
    starting_position: Position,
    starting_direction: Direction,
) -> Option<Vec<Position>> {
    let mut path = Vec::new();
    let mut position = starting_position;
    let mut direction = starting_direction;

    let mut step = 0;

    loop {
        // println!("{}", step);
        // display_board(map, &path);
        let vector = get_vector(direction);
        let new_position = move_pos(&position, &vector);

        if in_bounds(&new_position, bounds) {
            if is_obstacle(map, &new_position) {
                direction = get_next_direction(direction);
            } else {
                position = new_position;
                path.push(position);
            }
        } else {
            break;
        }

        step += 1;
        if step > 30000 {
            // display_board(map, &path);
            return None;
        }
    }

    Some(path)
}

pub fn part2() {
    let (map, bounds) = load_map("input/06/input.txt");
    let (start_position, start_direction) = get_started(&map).unwrap();
    let path = walk(&map, &bounds, start_position, start_direction).unwrap();
    let walkable_tiles = path.iter().collect::<HashSet<_>>();

    let mut sum = 0;

    for &(i, j) in walkable_tiles {
        let mut modified_map = map.clone();
        modified_map[i as usize][j as usize] = 'O';
        let modified_path = walk(&modified_map, &bounds, start_position, start_direction);
        let loop_created = modified_path.is_none();

        if loop_created {
            sum += 1;
        }
    }

    println!("day 6 part 2: {}", sum);
}
