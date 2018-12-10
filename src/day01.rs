day!(
    day01,
    "https://adventofcode.com/2018/day/1/input",
    part1,
    part2
);

use regex::Regex;
use std::collections::HashSet;

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Result<isize>> + 'a {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<s>\+|\-)(?P<v>\d+)").unwrap();
    };
    RE.captures_iter(input).map(|capture| {
        let delta: isize = capture["v"].parse()?;
        Ok(if &capture["s"] == "+" { delta } else { -delta })
    })
}

fn part1(input: &str) -> Result<isize> {
    let mut freq = 0;
    for input in parse_input(input) {
        freq += input?;
    }
    return Ok(freq);
}

fn part2(input: &str) -> Result<isize> {
    let mut seen = HashSet::new();
    let mut freq = 0;
    seen.insert(0);
    loop {
        for input in parse_input(input) {
            freq += input?;
            if !seen.insert(freq) {
                return Ok(freq);
            }
        }
    }
}

#[test]
fn day01_test() {
    assert_results!(part1,
        "+1, +1, +1" => 3,
        "+1, +1, -2" => 0,
        "-1, -2, -3" => -6,
    );
    assert_results!(part2,
        "+1, -1" => 0,
        "+3, +3, +4, -2, -4" => 10,
        "-6, +3, +8, +5, -6" => 5,
        "+7, +7, -2, -7, -4" => 14,
    );
}
