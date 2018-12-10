day!(
    day09,
    "https://adventofcode.com/2018/day/9/input",
    part1,
    part2
);

use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Counts {
    player: usize,
    marble: usize,
}

impl FromStr for Counts {
    type Err = Error;
    fn from_str(s: &str) -> Result<Counts> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<p>\d+) players; last marble is worth (?P<m>\d+) points$")
                    .unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or(Error::Input("invalid puzzle format"))?;
        let player = c["p"].parse::<usize>().unwrap();
        let marble = c["m"].parse::<usize>().unwrap() + 1;
        if player == 0 {
            return Err(Error::Input("player count may not be zero"));
        }
        Ok(Counts { player, marble })
    }
}

fn part1(input: &str) -> Result<u64> {
    let counts: Counts = input.parse()?;
    let mut player_scores = vec![0u64; counts.player];
    let mut marble_ring = VecDeque::with_capacity(counts.marble);
    marble_ring.push_back(0);

    let mut current_marble = 0;

    for index in 1..counts.marble {
        if index % 23 == 0 {
            let player_score = &mut player_scores[index % counts.player];
            *player_score += index as u64;

            current_marble = (marble_ring.len() + current_marble - 7) % marble_ring.len();
            *player_score += marble_ring.remove(current_marble).unwrap();

            continue;
        }
        current_marble = (current_marble + 2) % marble_ring.len();
        if current_marble == 0 {
            current_marble = marble_ring.len();
        }
        marble_ring.insert(current_marble, index as u64);
    }

    Ok(player_scores.into_iter().max().unwrap())
}

fn part2_impl(input: &str, should_multiply_input: bool) -> Result<u64> {
    let mut counts: Counts = input.parse()?;
    if should_multiply_input {
        counts.marble = (counts.marble - 1) * 100 + 1;
    }

    #[derive(Debug, Clone)]
    struct Marble {
        prev: usize,
        next: usize,
        value: u64,
    }
    let mut player_scores = vec![0u64; counts.player];
    let mut marbles = Vec::with_capacity(counts.marble);
    marbles.push(Marble {
        prev: 0,
        next: 0,
        value: 0,
    });

    let mut current_marble = 0;

    for index in 1..counts.marble {
        if index % 23 == 0 {
            let player_score = &mut player_scores[index % counts.player];
            *player_score += index as u64;

            // Move back 7 times
            for _ in 0..7 {
                current_marble = marbles[current_marble].prev;
            }

            // Add the score and remove the marble
            let to_be_removed = marbles[current_marble].clone();
            *player_score += to_be_removed.value;
            marbles[to_be_removed.prev].next = to_be_removed.next;
            marbles[to_be_removed.next].prev = to_be_removed.prev;

            current_marble = to_be_removed.next;
            continue;
        }

        // Move to the next marble
        current_marble = marbles[current_marble].next;

        // Insert after the current marble
        let new_marble = marbles.len();
        let prev_marble = current_marble;
        let next_marble = marbles[current_marble].next;
        marbles.push(Marble {
            prev: prev_marble,
            next: next_marble,
            value: index as u64,
        });
        marbles[prev_marble].next = new_marble;
        marbles[next_marble].prev = new_marble;
        current_marble = new_marble;
    }

    Ok(player_scores.into_iter().max().unwrap())
}

fn part2(input: &str) -> Result<u64> {
    part2_impl(input, true)
}

#[test]
fn day09_test() {
    fn test_both(input: &str) -> Result<u64> {
        let res1 = part1(input).unwrap();
        let res2 = part2_impl(input, false).unwrap();
        assert_eq!(res1, res2);
        Ok(res1)
    }

    assert_results!(test_both,
        "9 players; last marble is worth 25 points"    => 32,
        "10 players; last marble is worth 1618 points" => 8317,
        "13 players; last marble is worth 7999 points" => 146373,
        "17 players; last marble is worth 1104 points" => 2764,
        "21 players; last marble is worth 6111 points" => 54718,
        "30 players; last marble is worth 5807 points" => 37305,
    );
}
