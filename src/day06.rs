day!(
    day06,
    "https://adventofcode.com/2018/day/6/input",
    part1,
    part2
);

use crate::vec2::Vec2i;
use smallvec::SmallVec;

fn parse_input(input: &str) -> Result<Vec<Vec2i>> {
    input.lines().map(|x| Ok(x.parse()?)).collect()
}

#[derive(Debug)]
struct Point {
    pos: Vec2i,
    closest_to_count: usize,
}
fn compute_grid(points: &Vec<Vec2i>, padding: i32) -> Vec<Point> {
    fn find_closest_point(points: &Vec<Point>, pos: Vec2i) -> SmallVec<[usize; 4]> {
        let point_dists: SmallVec<[i32; 64]> = points
            .iter()
            .enumerate()
            .map(|(_, value)| {
                use num_traits::Signed;
                let abs_delta = pos.abs_sub(&value.pos);
                abs_delta.x + abs_delta.y
            })
            .collect();
        let min = *point_dists.iter().min().unwrap();
        point_dists
            .into_iter()
            .enumerate()
            .filter_map(|(index, v)| if v == min { Some(index) } else { None })
            .collect()
    }

    let min = Vec2i::new(
        points.iter().map(|v| v.x).min().unwrap() - padding,
        points.iter().map(|v| v.y).min().unwrap() - padding,
    );
    let max = Vec2i::new(
        points.iter().map(|v| v.x).max().unwrap() + padding,
        points.iter().map(|v| v.y).max().unwrap() + padding,
    );

    let mut points: Vec<_> = points
        .iter()
        .map(|&pos| Point {
            pos,
            closest_to_count: 0,
        })
        .collect();

    for x in min.x..max.x + 1 {
        for y in min.y..max.y + 1 {
            let pos = Vec2i::new(x, y);
            let mut closest = find_closest_point(&points, pos).into_iter();
            if let Some(index) = closest.next() {
                if let None = closest.next() {
                    points[index].closest_to_count += 1;
                }
            }
        }
    }
    points
}

fn part1(input: &str) -> Result<usize> {
    let input = parse_input(input)?;

    let base = compute_grid(&input, 0);
    let padded = compute_grid(&input, 1);

    base.into_iter()
        .zip(padded.into_iter())
        .filter_map(|(base, padded)| {
            assert_eq!(base.pos, padded.pos);
            if base.closest_to_count == padded.closest_to_count {
                Some(base.closest_to_count)
            } else {
                None
            }
        })
        .max()
        .ok_or(Error::Input("no non-infinite points"))
}

fn part2_impl(input: &str, max_distance: i32) -> Result<usize> {
    let points = parse_input(input)?;

    let min = Vec2i::new(
        points.iter().map(|v| v.x).min().unwrap(),
        points.iter().map(|v| v.y).min().unwrap(),
    );
    let max = Vec2i::new(
        points.iter().map(|v| v.x).max().unwrap(),
        points.iter().map(|v| v.y).max().unwrap(),
    );

    let mut in_range = 0;
    for x in min.x..max.x + 1 {
        for y in min.y..max.y + 1 {
            if points
                .iter()
                .map(|point| (x - point.x).abs() + (y - point.y).abs())
                .sum::<i32>() < max_distance
            {
                in_range += 1;
            }
        }
    }

    Ok(in_range)
}

fn part2(input: &str) -> Result<usize> {
    part2_impl(input, 10000)
}

#[test]
fn day06_test() {
    const EXAMPLE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    fn part2_test(input: &str) -> Result<usize> {
        part2_impl(input, 32)
    }

    assert_eq!(
        parse_input(EXAMPLE).unwrap(),
        vec![
            Vec2i::new(1, 1),
            Vec2i::new(1, 6),
            Vec2i::new(8, 3),
            Vec2i::new(3, 4),
            Vec2i::new(5, 5),
            Vec2i::new(8, 9),
        ]
    );

    assert_results!(part1, EXAMPLE => 17);
    assert_results!(part2_test, EXAMPLE => 16);
}
