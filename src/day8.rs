use std::{
    collections::{HashMap, HashSet},
    ops::{self, Range},
};

use crate::utils::load_2d_array;

type Map = Vec<Vec<char>>;
type MapBounds = (Range<i64>, Range<i64>);
type AntennaMap = HashMap<char, Vec<Point>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    i: i64,
    j: i64,
}
impl Point {
    fn from_usize(i: usize, j: usize) -> Point {
        Point {
            i: i as i64,
            j: j as i64,
        }
    }
}
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}
impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

fn load_map(path: &str) -> (Map, MapBounds) {
    let (map, map_width) = load_2d_array(path);
    let bounds = ((0..map.len() as i64), (0..map_width as i64));
    (map, bounds)
}

fn find_antennas(map: &Map) -> AntennaMap {
    let mut antennas: AntennaMap = HashMap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point::from_usize(i, j));
            }
        }
    }

    antennas
}

fn get_antinodes(p0: &Point, p1: &Point) -> (Point, Point) {
    ((*p1 - *p0) + *p1, (*p0 - *p1) + *p0)
}

fn in_bounds(bounds: &MapBounds, p: &Point) -> bool {
    bounds.0.contains(&p.i) && bounds.1.contains(&p.j)
}

fn display(map: &Map, antennas: &AntennaMap, antinodes: &HashSet<Point>) {
    let mut modified_map = map.clone();
    for antinode in antinodes {
        modified_map[antinode.i as usize][antinode.j as usize] = '#';
    }
    for (&c, antennas_of_a_type) in antennas {
        for antenna in antennas_of_a_type {
            modified_map[antenna.i as usize][antenna.j as usize] = c;
        }
    }
    for row in modified_map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

pub fn part1() {
    let (map, bounds) = load_map("input/08/input.txt");
    let antennas = find_antennas(&map);
    let mut antinodes = HashSet::new();
    for (_, antennas_of_a_type) in antennas.iter() {
        for (antenna_idx, antenna0) in antennas_of_a_type.iter().enumerate() {
            for antenna1 in antennas_of_a_type[antenna_idx + 1..antennas_of_a_type.len()].iter() {
                let (antinode0, antinode1) = get_antinodes(antenna0, antenna1);
                if in_bounds(&bounds, &antinode0) {
                    antinodes.insert(antinode0);
                }
                if in_bounds(&bounds, &antinode1) {
                    antinodes.insert(antinode1);
                }
            }
        }
    }
    display(&map, &antennas, &antinodes);
    println!("day 8 part 1: {}", antinodes.len());
}

fn get_resonant_antinodes(bounds: &MapBounds, p0: &Point, p1: &Point) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    antinodes.insert(*p0);
    antinodes.insert(*p1);

    let step = *p1 - *p0;
    let mut new_antinode = *p1 + step;
    while in_bounds(bounds, &new_antinode) {
        antinodes.insert(new_antinode);
        new_antinode = new_antinode + step;
    }

    let step = *p0 - *p1;
    let mut new_antinode = *p0 + step;
    while in_bounds(bounds, &new_antinode) {
        antinodes.insert(new_antinode);
        new_antinode = new_antinode + step;
    }

    antinodes
}

pub fn part2() {
    let (map, bounds) = load_map("input/08/input.txt");
    let antennas = find_antennas(&map);
    let mut antinodes = HashSet::new();
    for (_, antennas_of_a_type) in antennas.iter() {
        for (antenna_idx, antenna0) in antennas_of_a_type.iter().enumerate() {
            for antenna1 in antennas_of_a_type[antenna_idx + 1..antennas_of_a_type.len()].iter() {
                antinodes.extend(get_resonant_antinodes(&bounds, antenna0, antenna1));
            }
        }
    }
    display(&map, &antennas, &antinodes);
    println!("day 8 part 2: {}", antinodes.len());
}
