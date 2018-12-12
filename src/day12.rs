day!(
    day12,
    "https://adventofcode.com/2018/day/12/input",
    part1,
    part2
);

use smallvec::SmallVec;
use std::collections::{HashMap, VecDeque};
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    has_plants: Vec<bool>,
    index_offset: i64,
}
type Transformations = [bool; 32];

#[rustfmt::skip]
fn bools_to_nr(bools: [bool; 5]) -> u8 {
    #[inline(always)]
    fn n(b: bool) -> u8 { if b { 1 } else { 0 } }
    (n(bools[0]) << 4) |
    (n(bools[1]) << 3) |
    (n(bools[2]) << 2) |
    (n(bools[3]) << 1) |
    (n(bools[4]))
}

fn parse_input(input: &str) -> Result<(State, Transformations)> {
    fn char_to_bool(c: char) -> Result<bool> {
        match c {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => Err(Error::Input("invalid chars in initial state")),
        }
    }

    let mut lines = input.lines();
    let mut initial_state = lines.next().ok_or(Error::Input("empty input"))?;
    if !initial_state.starts_with("initial state: ") {
        return Err(Error::Input("expected initial state"));
    }
    initial_state = &initial_state[15..];
    let state = State {
        has_plants: initial_state
            .chars()
            .map(char_to_bool)
            .collect::<Result<_>>()?,
        index_offset: 0,
    };

    if lines.next() != Some("") {
        return Err(Error::Input(
            "expected empty line between initial state and transformations",
        ));
    }

    let mut transformations: Transformations = [false; 32];
    while let Some(line) = lines.next() {
        let line = line.as_bytes();
        if line.len() != 10 || !line.iter().all(|b| *b < 128) || &line[5..9] != b" => " {
            return Err(Error::Input("invalid transformation"));
        }
        let input = [
            char_to_bool(line[0] as char)?,
            char_to_bool(line[1] as char)?,
            char_to_bool(line[2] as char)?,
            char_to_bool(line[3] as char)?,
            char_to_bool(line[4] as char)?,
        ];
        let output = char_to_bool(line[9] as char)?;
        transformations[bools_to_nr(input) as usize] = output;
    }

    Ok((state, transformations))
}

fn process_n(n: usize, state: State, transformations: &Transformations) -> State {
    if n == 0 {
        return state;
    }
    let index_offset = state.index_offset - (n as i64) * 2;

    // Expand the working area by 2 for each process
    let padding = iter::repeat(false).take(n * 2);
    let mut fore_buffer = padding
        .clone()
        .chain(state.has_plants.into_iter())
        .chain(padding)
        .collect::<Vec<_>>();

    let mut back_buffer = fore_buffer.clone();
    for _ in 0..n {
        // Transform state from fore_buffer to back_buffer
        for i in 2..fore_buffer.len() - 2 {
            let transformation_index = bools_to_nr([
                fore_buffer[i - 2],
                fore_buffer[i - 1],
                fore_buffer[i],
                fore_buffer[i + 1],
                fore_buffer[i + 2],
            ]);
            back_buffer[i] = transformations[transformation_index as usize];
        }

        // Swap fore and back
        std::mem::swap(&mut fore_buffer, &mut back_buffer);
    }

    State {
        has_plants: fore_buffer,
        index_offset,
    }
}

fn sum_state(state: &State) -> i64 {
    state
        .has_plants
        .iter()
        .enumerate()
        .filter_map(|(i, has_plant)| {
            if *has_plant {
                Some(i as i64 + state.index_offset)
            } else {
                None
            }
        })
        .sum()
}

fn part1(input: &str) -> Result<i64> {
    let (state, transformations) = parse_input(input)?;
    let state = process_n(20, state, &transformations);

    Ok(sum_state(&state))
}

fn compress_bools(vec: &VecDeque<bool>) -> SmallVec<[u64; 4]> {
    let len = (vec.len() + 63) / 64; // ceil(len / 64)
    let mut res = SmallVec::with_capacity(len);
    for i in 0..vec.len() {
        let base = i / 64;
        let offset = i % 64;
        let value = vec[i];
        if offset == 0 {
            res.push(if value { 1 } else { 0 });
        } else if value {
            res[base] |= 1 << offset;
        }
    }

    res
}

fn part2(input: &str) -> Result<i64> {
    const TARGET_ITERATIONS: u64 = 50000000000;
    let (state, transformations) = parse_input(input)?;

    let mut index_offset = state.index_offset;
    let mut fore_buffer: VecDeque<bool> = state.has_plants.into_iter().collect();
    fn trim(vec: &mut VecDeque<bool>, index_offset: &mut i64) {
        while let Some(&false) = vec.front() {
            vec.pop_front();
            *index_offset += 1;
        }
        while let Some(&false) = vec.back() {
            vec.pop_back();
        }
    }
    let mut back_buffer = fore_buffer.clone();

    let mut history = HashMap::new();
    history.insert(compress_bools(&fore_buffer), (0, index_offset));

    for iteration_nr in 1u64.. {
        // Pad out the fore_buffer with 4 false' on each side
        fore_buffer.resize(fore_buffer.len() + 4, false);
        for _ in 0..4 {
            fore_buffer.push_front(false);
        }
        index_offset -= 4;

        // Write the data into the back_buffer
        let len = fore_buffer.len();
        back_buffer.resize(len, false);
        back_buffer[0] = false;
        back_buffer[1] = false;
        back_buffer[len - 2] = false;
        back_buffer[len - 1] = false;
        for i in 2..len - 2 {
            let transformation_index = bools_to_nr([
                fore_buffer[i - 2],
                fore_buffer[i - 1],
                fore_buffer[i],
                fore_buffer[i + 1],
                fore_buffer[i + 2],
            ]);
            back_buffer[i] = transformations[transformation_index as usize];
        }

        // Swap and trim
        std::mem::swap(&mut fore_buffer, &mut back_buffer);
        trim(&mut fore_buffer, &mut index_offset);

        // Check if the current state is the same as a previous state
        if let Some((previous_iteration_nr, previous_index_offset)) =
            history.insert(compress_bools(&fore_buffer), (iteration_nr, index_offset))
        {
            let cycle_length = iteration_nr - previous_iteration_nr;
            let full_cycles_left = (TARGET_ITERATIONS - iteration_nr) / cycle_length;
            let remaining_iterations = (TARGET_ITERATIONS - iteration_nr) % cycle_length;
            let state = State {
                has_plants: fore_buffer.into_iter().collect(),
                index_offset: index_offset
                    + (index_offset - previous_index_offset) * (full_cycles_left as i64),
            };
            let state = process_n(remaining_iterations as usize, state, &transformations);
            return Ok(sum_state(&state));
        }
    }

    unreachable!()
}

#[test]
fn day12_test() {
    const EXAMPLE: &str = r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    assert_results!(part1, EXAMPLE => 325);
}
