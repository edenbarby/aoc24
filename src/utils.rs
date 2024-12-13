use std::{
    fs::read_to_string,
    ops::{self, Range},
};

pub fn extract_ints_from_string(s: &str) -> Vec<i32> {
    s.split(' ')
        .skip_while(|t| t.is_empty())
        .map(|t| t.parse::<i32>().unwrap())
        .collect()
}

pub fn load_pair_of_columns(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut input_left = Vec::new();
    let mut input_right = Vec::new();

    for line in read_to_string(filename).unwrap().trim().lines() {
        let mut i = 0;
        for token in line.split(' ') {
            if !token.is_empty() {
                let num = token.parse::<i32>().unwrap();
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

pub fn load_2d_array(path: &str) -> (Vec<Vec<char>>, usize) {
    let mut output: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(path).unwrap().trim().lines() {
        output.push(line.chars().collect());
    }

    assert!(!output.is_empty());
    let size_inner = output[0].len();
    for inner in &output[1..] {
        assert_eq!(inner.len(), size_inner);
    }

    (output, size_inner)
}

pub fn load_map(path: &str) -> (Map, MapBounds) {
    let (map, map_width) = load_2d_array(path);
    let bounds = ((0..map.len() as i64), (0..map_width as i64));
    (map, bounds)
}

type Map = Vec<Vec<char>>;
type MapBounds = (Range<i64>, Range<i64>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub i: i64,
    pub j: i64,
}
impl Point {
    fn from_usize(i: usize, j: usize) -> Point {
        Point {
            i: i as i64,
            j: j as i64,
        }
    }
    pub fn adjacent_4(self) -> [Point; 4] {
        [
            Point {
                i: self.i - 1,
                j: self.j - 1,
            },
            Point {
                i: self.i - 1,
                j: self.j + 1,
            },
            Point {
                i: self.i + 1,
                j: self.j - 1,
            },
            Point {
                i: self.i + 1,
                j: self.j + 1,
            },
        ]
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

pub fn in_bounds(bounds: &MapBounds, p: &Point) -> bool {
    bounds.0.contains(&p.i) && bounds.1.contains(&p.j)
}
