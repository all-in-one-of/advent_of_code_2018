day!(
    day05,
    "https://adventofcode.com/2018/day/5/input",
    part1,
    part2
);

use regex::Regex;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::str::FromStr;

fn transform_input(input: &str) -> Result<Vec<u8>> {
    for &b in input.as_bytes() {
        if b < 0x20 || b > 0x7f {
            return Err(Error::Input("invalid characters in input"));
        }
    }
    Ok(input.bytes().rev().collect())
}

// Optimized to apply multiple reactions per iteration
// as well as batch the removals.
fn react_fully(data: &mut Vec<u8>) -> usize {
    let mut count = 0;
    loop {
        let n = react_multiple(data);
        if n == 0 {
            return count;
        }
        count += n;
    }
}

fn react_multiple(data: &mut Vec<u8>) -> usize {
    let mut react_points: SmallVec<[usize; 64]> = data
        .iter()
        .cloned()
        .zip(data.iter().skip(1).cloned())
        .enumerate()
        .filter_map(|(i, (a, b))| {
            let a = a as char;
            let b = b as char;
            if a.to_ascii_lowercase() == b.to_ascii_lowercase()
                && a.is_ascii_lowercase() != b.is_ascii_lowercase()
            {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    if react_points.len() == 0 {
        return 0;
    }

    // Example: aAaABbB
    // react_points: [0, 1, 2, 4, 5]
    // Will result in:
    // overlaps: [1, 5]
    // react_points: [0, 2, 4]
    let mut overlaps: SmallVec<[usize; 32]> = SmallVec::new();
    let mut i = 0;
    while i < react_points.len() - 1 {
        let mut j = 1;
        while i + j < react_points.len() && react_points[i] + j == react_points[i + j] {
            if j % 2 == 1 {
                overlaps.push(i + j);
            }
            j += 1;
        }
        i += j;
    }
    for &overlap in overlaps.iter().rev() {
        react_points.remove(overlap);
    }

    let mut data_idx = 0;
    let mut react_idx = 0;
    data.drain_filter(|_| {
        let n = data_idx;
        data_idx += 1;
        if react_idx < react_points.len() {
            // first character
            if react_points[react_idx] == n {
                return true;
            // second character
            } else if react_points[react_idx] + 1 == n {
                react_idx += 1;
                return true;
            }
        }
        false
    });
    react_points.len()
}

// Naive approach
#[cfg(test)]
fn react(data: &mut Vec<u8>) -> bool {
    if let Some(index) = data
        .iter()
        .cloned()
        .zip(data.iter().skip(1).cloned())
        .enumerate()
        .rev()
        .filter_map(|(i, (a, b))| {
            let a = a as char;
            let b = b as char;
            if a.to_ascii_lowercase() == b.to_ascii_lowercase()
                && a.is_ascii_lowercase() != b.is_ascii_lowercase()
            {
                Some(i)
            } else {
                None
            }
        })
        .next()
    {
        data.drain(index..index + 2);
        true
    } else {
        false
    }
}

fn part1(input: String) -> Result<usize> {
    let mut data = transform_input(&input)?;
    react_fully(&mut data);
    Ok(data.len())
}

fn part2(input: String) -> Result<usize> {
    let data = transform_input(&input)?;
    data.iter()
        .map(|&c| (c as char).to_ascii_lowercase())
        .collect::<HashSet<char>>()
        .into_iter()
        .map(|lower| {
            let upper = lower.to_ascii_uppercase() as u8;
            let lower = lower as u8;
            let mut data = data.clone();
            data.drain_filter(|&mut x| x == lower || x == upper);
            react_fully(&mut data);
            data.len()
        })
        .min()
        .ok_or(Error::Input("empty input"))
}

#[test]
fn day05_test() {
    let mut s = transform_input("dabAcCaCBAcCcaDA").unwrap();
    assert_eq!(react_fully(&mut s), 3);
    assert_eq!(s, transform_input("dabCBAcaDA").unwrap());

    let mut s = transform_input("dabAcCaCBAcCcaDA").unwrap();
    assert!(react(&mut s));
    assert_eq!(s, transform_input("dabAaCBAcCcaDA").unwrap());
    assert!(react(&mut s));
    assert_eq!(s, transform_input("dabCBAcCcaDA").unwrap());
    assert!(react(&mut s));
    assert_eq!(s, transform_input("dabCBAcaDA").unwrap());
    assert!(!react(&mut s));
    assert_eq!(s, transform_input("dabCBAcaDA").unwrap());

    assert_results!(part1, "dabAcCaCBAcCcaDA" => 10);
    assert_results!(part2, "dabAcCaCBAcCcaDA" => 4);
}
