day!(
    day03,
    "https://adventofcode.com/2018/day/3/input",
    part1,
    part2
);

use regex::Regex;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Claim {
    id: usize,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = Error;
    fn from_str(s: &str) -> Result<Claim> {
        // #1 @ 1,3: 4x4
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)$"
            )
            .unwrap();
        };
        let capture = RE.captures(s).ok_or(Error::Input("invalid claim"))?;
        Ok(Claim {
            id: capture["id"].parse()?,
            top: capture["top"].parse()?,
            left: capture["left"].parse()?,
            width: capture["width"].parse()?,
            height: capture["height"].parse()?,
        })
    }
}

type RequestGrid = Vec<SmallVec<[usize; 4]>>;
fn get_request_grid(claims: &Vec<Claim>) -> RequestGrid {
    let mut grid: RequestGrid = Vec::with_capacity(1000 * 1000);
    for _ in 0..1000 * 1000 {
        grid.push(SmallVec::new());
    }

    for claim in claims {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                assert!(x < 1000 && y < 1000);
                grid[x * 1000 + y].push(claim.id);
            }
        }
    }
    grid
}

fn part1(input: &str) -> Result<usize> {
    let claims = input
        .lines()
        .map(Claim::from_str)
        .collect::<Result<Vec<_>>>()?;

    let req_grid = get_request_grid(&claims);
    Ok(req_grid.iter().filter(|x| x.len() > 1).count())
}

fn part2(input: &str) -> Result<usize> {
    let claims = input
        .lines()
        .map(Claim::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut non_overlapping_ids = claims.iter().map(|x| x.id).collect::<HashSet<_>>();
    let req_grid = get_request_grid(&claims);

    for overlapping_cell in req_grid.iter().filter(|x| x.len() > 1) {
        for overlapping_id in overlapping_cell {
            non_overlapping_ids.remove(overlapping_id);
        }
    }

    if non_overlapping_ids.len() != 1 {
        return Err(Error::Input("multiple non-overlapping ids"));
    }
    non_overlapping_ids
        .into_iter()
        .next()
        .ok_or(Error::Input("expected one item without overlap"))
}

#[test]
fn day03_test() {
    const EXAMPLE: &'static str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
    assert_results!(part1, EXAMPLE => 4);
    assert_results!(part2, EXAMPLE => 3);
}
