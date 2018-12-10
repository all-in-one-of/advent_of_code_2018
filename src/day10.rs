day!(
    day10,
    "https://adventofcode.com/2018/day/10/input",
    part1,
    part2
);

use crate::vec2::{AabbIteratorEx, Vec2i};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    position: Vec2i,
    velocity: Vec2i,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Point> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^position=<(?P<p>.+?)> velocity=<(?P<v>.+?)>$").unwrap();
        }

        let c = RE.captures(s).ok_or(Error::Input("invalid input format"))?;
        Ok(Point {
            position: c["p"].parse()?,
            velocity: c["v"].parse()?,
        })
    }
}

fn points_to_str(points: &[Point]) -> String {
    let positions = points.iter().map(|p| p.position);
    let (min, max) = positions.clone().aabb().unwrap();
    let size = max - min + Vec2i::new(1, 1);
    let mut res = Vec::with_capacity(((size.x + 1) * size.y) as usize);
    for _ in 0..size.y {
        for _ in 0..size.x {
            res.push(b'.');
        }
        res.push(b'\n');
    }
    res.pop();

    for position in positions {
        let offset = position - min;
        res[(offset.x + offset.y * (size.x + 1)) as usize] = b'#';
    }

    String::from_utf8(res).unwrap()
}

fn part_both(input: &str) -> Result<(usize, String)> {
    let mut points_fore = input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<Point>>>()?;
    let mut points_back = points_fore.clone();

    let mut last_aabb_size = {
        let (min, max) = points_fore
            .iter()
            .map(|p| p.position)
            .aabb()
            .ok_or(Error::Input("no puzzle input"))?;
        max - min
    };
    for time in 0..1000000 {
        // Transform point from fore to back
        for (i, p) in points_fore.iter().enumerate() {
            points_back[i] = Point {
                position: p.position + p.velocity,
                velocity: p.velocity,
            };
        }

        // Check the new AABB size
        let current_aabb_size = {
            let (min, max) = points_back.iter().map(|p| p.position).aabb().unwrap();
            max - min
        };

        // Check if the new AABB is larger, and if so, return the previous result
        if current_aabb_size.x + current_aabb_size.y > last_aabb_size.x + last_aabb_size.y {
            return Ok((time, points_to_str(&points_fore)));
        }

        // Store and swap
        last_aabb_size = current_aabb_size;
        std::mem::swap(&mut points_fore, &mut points_back);
    }

    Err(Error::Input("cannot solve input in a million iterations"))
}

fn part1(input: &str) -> Result<String> {
    part_both(input).map(|(_, s)| s)
}

fn part2(input: &str) -> Result<usize> {
    part_both(input).map(|(t, _)| t)
}

#[test]
fn day10_test() {
    assert_eq!(
        "position=<-6, 10> velocity=< 2, -2>"
            .parse::<Point>()
            .unwrap(),
        Point {
            position: Vec2i::new(-6, 10),
            velocity: Vec2i::new(2, -2),
        }
    );

    const EXAMPLE: &str = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
    const INITIAL: &str = r"........#.............
................#.....
.........#.#..#.......
......................
#..........#.#.......#
...............#......
....#.................
..#.#....#............
.......#..............
......#...............
...#...#.#...#........
....#..#..#.........#.
.......#..............
...........#..#.......
#...........#.........
...#.......#..........";
    const OUTPUT: &str = r"#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###";

    assert_eq!(
        INITIAL,
        points_to_str(
            &EXAMPLE
                .lines()
                .map(Point::from_str)
                .collect::<Result<Vec<_>>>()
                .unwrap()
        )
    );

    assert_results!(part1, EXAMPLE => OUTPUT);
    assert_results!(part2, EXAMPLE => 3);
}
