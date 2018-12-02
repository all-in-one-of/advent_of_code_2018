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

    if input.len() == 0 {
        return Err(Error::Input("empty input"));
    }
    let str_length = input[0].len();
    if !input.iter().skip(1).all(|x| x.len() == str_length) {
        return Err(Error::Input("not all strings have the same length"));
    }

    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let pairs = input[i].chars().zip(input[j].chars());

            let differences = pairs.clone().filter(|&(a, b)| a != b);
            if differences.take(2).count() != 1 {
                continue;
            }

            return Ok(pairs
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect());
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
