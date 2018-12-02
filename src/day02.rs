day!(
    day02,
    "https://adventofcode.com/2018/day/2/input",
    part1,
    part2
);

use std::collections::HashMap;

fn letter_counts(input: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in input.chars() {
        *map.entry(c).or_default() += 1;
    }
    map
}

fn part1(input: String) -> Result<isize> {
    let mut twos = 0;
    let mut threes = 0;
    for count in input.lines().map(letter_counts) {
        if count.iter().any(|(_, &value)| value == 2) {
            twos += 1;
        }
        if count.iter().any(|(_, &value)| value == 3) {
            threes += 1;
        }
    }
    return Ok(twos * threes);
}

fn part2(input: String) -> Result<String> {
    let input = input.lines().collect::<Vec<&str>>();
    let char_count = input[0].len();

    if char_count == 0 {
        return Err(Error::Input("empty input"));
    }
    if !input.iter().skip(1).all(|x| x.len() == char_count) {
        return Err(Error::Input("not all strings have the same length"));
    }

    for i in 0..input.len() - 1 {
        'j: for j in i..input.len() {
            let differences = (0..char_count)
                .filter(|&index| input[i].as_bytes()[index] != input[j].as_bytes()[index]);
            // Check if there is exactly 1 difference, take 2 avoids fully iterating
            if differences.take(2).count() == 1 {
                let mut res = String::with_capacity(char_count - 1);
                res.extend((0..char_count).filter_map(|index| {
                    let a = input[i].as_bytes()[index];
                    let b = input[j].as_bytes()[index];
                    if a == b {
                        Some(a as char)
                    } else {
                        None
                    }
                }));
                return Ok(res);
            }
        }
    }
    Err(Error::Input("no single delta found"))
}

#[test]
fn day02_test() {
    assert_results!(part1,
        "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab" => 12,
);
    assert_results!(part2,
        "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz" => "fgij",
    );
}
