day!(
    day23,
    "https://adventofcode.com/2018/day/23/input",
    part1,
    part2
);

use crate::vec3::AabbIteratorEx;
use num_traits::{One, Signed, Zero};
use regex::Regex;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;
use std::str::FromStr;

type Vec3 = crate::vec3::Vec3i;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Nanobot {
    pos: Vec3,
    radius: i32,
}

impl Nanobot {
    fn distance(&self, other: &Nanobot) -> i32 {
        let v = self.pos.abs_sub(&other.pos);
        v.x + v.y + v.z
    }
    fn radius_contains(&self, other: &Nanobot) -> bool {
        self.distance(other) <= self.radius
    }
}

impl FromStr for Nanobot {
    type Err = Error;
    fn from_str(s: &str) -> Result<Nanobot> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^pos=<(\-?\d+),(\-?\d+),(\-?\d+)>, r=(\-?\d+)$").unwrap();
        }

        let c = RE
            .captures(s)
            .ok_or(Error::Input("invalid nanobot string"))?;
        Ok(Nanobot {
            pos: Vec3::new(
                c[1].parse().unwrap(),
                c[2].parse().unwrap(),
                c[3].parse().unwrap(),
            ),
            radius: c[4].parse().unwrap(),
        })
    }
}

fn part1(input: &str) -> Result<usize> {
    let nanobots = input
        .lines()
        .map(Nanobot::from_str)
        .collect::<Result<Vec<_>>>()?;
    let largest_radius = nanobots
        .iter()
        .max_by_key(|nanobot| nanobot.radius)
        .ok_or(Error::Input("empty input"))?;
    Ok(nanobots
        .iter()
        .filter(|nanobot| largest_radius.radius_contains(nanobot))
        .count())
}

fn round_up_to_next_power_of_2(mut v: u32) -> u32 {
    // http://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
    assert!(v <= 0x4000_0000);
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    v
}

fn part2(input: &str) -> Result<i32> {
    let nanobots = &input
        .lines()
        .map(Nanobot::from_str)
        .collect::<Result<Vec<_>>>()?;

    if nanobots.is_empty() {
        return Err(Error::Input("empty input"));
    }

    let (min_corner, initial_size) = {
        let (min, max) = nanobots.iter().map(|n| n.pos).aabb().unwrap();
        let size = Vec3::one() + max - min;

        let max_dimension = size.x.max(size.y.max(size.z));
        let initial_size = round_up_to_next_power_of_2(max_dimension as u32) as i32;
        (min, initial_size)
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Cube {
        pos: Vec3,
        size: i32,
        intersections: i32,
    }

    impl Ord for Cube {
        fn cmp(&self, other: &Self) -> Ordering {
            self.intersections.cmp(&other.intersections)
        }
    }
    impl PartialOrd for Cube {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut found_intersection_count = None;
    let mut found_cells = Vec::new();
    let mut cube_heap = BinaryHeap::new();
    cube_heap.push(Cube {
        pos: min_corner,
        size: initial_size,
        // the initial cube by definition contains all nanobots
        intersections: nanobots.len() as i32,
    });
    debug_assert_eq!(
        nanobots.len(),
        make_cube(nanobots, min_corner, initial_size).intersections as usize
    );

    fn make_cube(nanobots: &Vec<Nanobot>, pos: Vec3, size: i32) -> Cube {
        let mut intersections = 0;
        let size_vec = Vec3::one() * (size - 1);
        for nanobot in nanobots {
            let offset = (pos - nanobot.pos) * 2 + size_vec;
            let distance = (offset.abs() - size_vec).max(Vec3::zero());
            let radius = nanobot.radius * 2;
            if distance.x + distance.y + distance.z <= radius {
                intersections += 1;
            }
        }

        Cube {
            pos,
            size,
            intersections,
        }
    }

    loop {
        let cube = match cube_heap.pop() {
            None => break,
            Some(v) => v,
        };

        if let Some(found_intersection_count) = found_intersection_count {
            if cube.intersections < found_intersection_count {
                break;
            }
        }

        if cube.size == 1 {
            if let Some(found_intersection_count) = found_intersection_count {
                if cube.intersections != found_intersection_count {
                    found_cells.clear();
                }
            }
            found_intersection_count = Some(cube.intersections);
            found_cells.push(cube.pos);
            continue;
        }

        // Split up the cube in 8 octants
        let p = cube.pos;
        let s = cube.size / 2;
        cube_heap.push(make_cube(nanobots, p + Vec3::new(0, 0, 0), s));
        cube_heap.push(make_cube(nanobots, p + Vec3::new(0, 0, s), s));
        cube_heap.push(make_cube(nanobots, p + Vec3::new(0, s, 0), s));
        cube_heap.push(make_cube(nanobots, p + Vec3::new(0, s, s), s));
        cube_heap.push(make_cube(nanobots, p + Vec3::new(s, 0, 0), s));
        cube_heap.push(make_cube(nanobots, p + Vec3::new(s, 0, s), s));
        cube_heap.push(make_cube(nanobots, p + Vec3::new(s, s, 0), s));
        cube_heap.push(make_cube(nanobots, p + Vec3::new(s, s, s), s));
    }

    Ok(found_cells
        .into_iter()
        .map(|v| v.x.abs() + v.y.abs() + v.z.abs())
        .min()
        .unwrap())
}

#[test]
fn day23_test() {
    assert_results!(part1, "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1" => 7);
    assert_results!(part2, "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5" => 36);
}
