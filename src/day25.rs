day!(
    day25,
    "https://adventofcode.com/2018/day/25/input",
    part1,
    !
);

use std::cell::RefCell;
use std::collections::hash_map::{Entry, HashMap};
use std::iter::repeat;
use std::mem::swap;

type Point = [i32; 4];

fn parse_input(input: &str) -> Result<Vec<Point>> {
    let points = input
        .lines()
        .map(|line| {
            let mut i = line.split(',');
            let p: Point = [
                i.next()
                    .ok_or(Error::Input("expected 4 numbers, got 0"))?
                    .parse()?,
                i.next()
                    .ok_or(Error::Input("expected 4 numbers, got 1"))?
                    .parse()?,
                i.next()
                    .ok_or(Error::Input("expected 4 numbers, got 2"))?
                    .parse()?,
                i.next()
                    .ok_or(Error::Input("expected 4 numbers, got 3"))?
                    .parse()?,
            ];
            if i.next().is_some() {
                Err(Error::Input("expected end of line after 4 numbers"))
            } else {
                Ok(p)
            }
        })
        .collect::<Result<Vec<_>>>()?;
    if points.len() < 2 {
        return Err(Error::Input("expected at least 2 points"));
    }
    Ok(points)
}

#[rustfmt::skip]
fn manhathan_distance(a: &Point, b: &Point) -> i32 {
    i32::abs(a[0] - b[0]) +
    i32::abs(a[1] - b[1]) +
    i32::abs(a[2] - b[2]) +
    i32::abs(a[3] - b[3])
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Constellation {
    points: Vec<Point>,
}

fn part1(input: &str) -> Result<usize> {
    let points = parse_input(input)?;
    let mut constellations: Vec<Constellation> = Vec::new();
    let mut map: Vec<Option<usize>> = repeat(None).take(points.len()).collect();

    // Build constellations
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if manhathan_distance(&points[i], &points[j]) > 3 {
                continue;
            }

            let i_exists = map[i].is_some();
            let j_exists = map[j].is_some();
            // Create new constellation
            if !i_exists && !j_exists {
                let index = constellations.len();
                constellations.push(Constellation {
                    points: vec![points[i], points[j]],
                });
                map[i] = Some(index);
                map[j] = Some(index);
            }
            // Add point i to constellation j, or point j to constellation i
            else if !(i_exists && j_exists) {
                let (constellation, point) = if i_exists { (map[i].unwrap(), j) } else { (map[j].unwrap(), i) };
                constellations[constellation].points.push(points[point]);
                map[point] = Some(constellation);
            }
            // Merge constellations, merge the higher index into the lower index
            else {
                let mut a = map[i].unwrap();
                let mut b = map[j].unwrap();
                // Special case where they're both already in the same constellation
                if a == b {
                    continue;
                }
                // Ensure that a is smaller than b
                // Note that i and j no longer match up with a and b after this
                if a > b {
                    swap(&mut a, &mut b);
                }

                // Remove the constellation and update the mapping
                let removed_constellation = constellations.remove(b);
                for index in &mut map {
                    if let Some(index) = index.as_mut() {
                        if *index == b {
                            *index = a;
                        } else if *index > b {
                            *index -= 1;
                        }
                    }
                }

                // Merge the points
                constellations[a].points.extend(removed_constellation.points);
            }
        }
    }

    // Create single-point constellations
    for i in 0..points.len() {
        if map[i].is_none() {
            map[i] = Some(constellations.len());
            constellations.push(Constellation {
                points: vec![points[i]],
            });
        }
    }

    Ok(constellations.len())
}

#[test]
fn day25_test() {
    assert_results!(part1, "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0" => 2, "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
6,0,0,0" => 1, "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0" => 4, "\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2" => 3, "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2" => 8);
}
