day!(
    day14,
    "https://adventofcode.com/2018/day/14/input",
    part1,
    part2
);

use twoway::find_bytes;

#[derive(Debug, Clone)]
struct State {
    recipes: Vec<u8>,
    cursor1: usize,
    cursor2: usize,
}

impl State {
    fn new() -> State {
        State {
            recipes: vec![3, 7],
            cursor1: 0,
            cursor2: 1,
        }
    }

    fn progress(&mut self) {
        let value1 = self.recipes[self.cursor1];
        let value2 = self.recipes[self.cursor2];

        // Create new recipes
        let new_recipes = value1 + value2;
        if new_recipes >= 10 {
            self.recipes.push(new_recipes / 10);
            self.recipes.push(new_recipes % 10);
        } else {
            self.recipes.push(new_recipes);
        }

        // Progress cursor
        self.cursor1 = (self.cursor1 + value1 as usize + 1) % self.recipes.len();
        self.cursor2 = (self.cursor2 + value2 as usize + 1) % self.recipes.len();
    }
}

fn part1(input: &str) -> Result<String> {
    let input = input.parse::<usize>()?;
    let mut state = State::new();

    loop {
        if state.recipes.len() >= 10 + input {
            break;
        }

        state.progress();
    }

    let mut res = String::with_capacity(10);
    for i in input..input + 10 {
        res.push((state.recipes[i] + b'0') as char);
    }
    Ok(res)
}
fn part2(input: &str) -> Result<usize> {
    let length = input.len();
    let mut search_target = Vec::with_capacity(length);
    for i in 0..length {
        let byte = input.as_bytes()[i];
        if byte < b'0' || byte > b'9' {
            return Err(Error::Input("expected digits as input"));
        }
        search_target.push(byte - b'0');
    }

    let mut state = State::new();
    while state.recipes.len() < length {
        state.progress();
    }

    let mut checked_from_index = 0;
    for _ in 0..100_000_000 {
        state.progress();

        if let Some(offset) = find_bytes(&state.recipes[checked_from_index..], &search_target) {
            return Ok(offset + checked_from_index);
        }
        checked_from_index = state.recipes.len() + 1 - length;
    }

    Err(Error::Input("cannot solve in a 100 million iterations"))
}

#[test]
fn day14_test() {
    assert_results!(part1,
        "9"    => "5158916779",
        "5"    => "0124515891",
        "18"   => "9251071085",
        "2018" => "5941429882",
    );
    assert_results!(part2,
        "51589" => 9,
        "01245" => 5,
        "92510" => 18,
        "59414" => 2018,
    );
}
