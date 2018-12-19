day!(
    day18,
    "https://adventofcode.com/2018/day/18/input",
    part1,
    part2
);

use smallvec::SmallVec;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

type Vec2 = crate::vec2::Vec2us;
type Mat2 = crate::mat2::Mat2<Cell>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CollectionArea {
    cells: Mat2,
}

impl FromStr for CollectionArea {
    type Err = Error;
    fn from_str(s: &str) -> Result<CollectionArea> {
        let mut lines = s.lines().peekable();
        let size = lines.peek().ok_or(Error::Input("empty input"))?.len();
        let mut cells = Mat2::new(Cell::Open, Vec2::new(size, size));
        let mut line_count = 0;
        for (line_idx, line) in lines.enumerate() {
            line_count += 1;
            if line_idx >= size {
                return Err(Error::Input("too many lines"));
            }
            if size != line.len() {
                return Err(Error::Input("inconsistent width"));
            }

            for (x, c) in line.chars().enumerate() {
                cells[x][line_idx] = match c {
                    '.' => Cell::Open,
                    '|' => Cell::Trees,
                    '#' => Cell::Lumberyard,
                    _ => return Err(Error::Input("invalid character")),
                };
            }
        }
        if line_count != size {
            return Err(Error::Input("too few lines"));
        }

        Ok(CollectionArea { cells })
    }
}
impl Display for CollectionArea {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::with_capacity((self.cells.width() + 1) * self.cells.height());
        for y in 0..self.cells.height() {
            for x in 0..self.cells.width() {
                s.push(match self.cells[x][y] {
                    Cell::Open => '.',
                    Cell::Trees => '|',
                    Cell::Lumberyard => '#',
                });
            }
            s.push('\n');
        }
        s.pop();

        <String as Display>::fmt(&s, f)
    }
}

impl CollectionArea {
    fn neighbors(&self, position: Vec2) -> SmallVec<[Vec2; 8]> {
        let mut neighbors = SmallVec::new();
        let size = self.cells.size();

        if position.x > 0 {
            if position.y > 0 {
                neighbors.push(Vec2::new(position.x - 1, position.y - 1)); // top left
            }
            neighbors.push(position.with_x(position.x - 1)); // left
            if position.y + 1 < size.y {
                neighbors.push(Vec2::new(position.x - 1, position.y + 1)); // bottom left
            }
        }
        if position.y > 0 {
            neighbors.push(Vec2::new(position.x, position.y - 1)); // top
        }
        if position.y + 1 < size.y {
            neighbors.push(Vec2::new(position.x, position.y + 1)); // bottom
        }
        if position.x + 1 < size.x {
            if position.y > 0 {
                neighbors.push(Vec2::new(position.x + 1, position.y - 1)); // top right
            }
            neighbors.push(position.with_x(position.x + 1)); // right
            if position.y + 1 < size.y {
                neighbors.push(Vec2::new(position.x + 1, position.y + 1)); // bottom right
            }
        }

        neighbors
    }

    fn update_into(&self, target: &mut CollectionArea) {
        assert_eq!(self.cells.size(), target.cells.size());
        for x in 0..self.cells.width() {
            let column = &self.cells[x];
            for y in 0..self.cells.height() {
                let position = Vec2::new(x, y);
                let neighbors = self.neighbors(position);
                target.cells[position] = match column[y] {
                    Cell::Open => {
                        if neighbors
                            .into_iter()
                            .filter(|p| self.cells[*p] == Cell::Trees)
                            .count()
                            >= 3
                        {
                            Cell::Trees
                        } else {
                            Cell::Open
                        }
                    }
                    Cell::Trees => {
                        if neighbors
                            .into_iter()
                            .filter(|p| self.cells[*p] == Cell::Lumberyard)
                            .count()
                            >= 3
                        {
                            Cell::Lumberyard
                        } else {
                            Cell::Trees
                        }
                    }
                    Cell::Lumberyard => {
                        let mut has_trees = false;
                        let mut has_lumberyard = false;
                        for neighbor in neighbors {
                            match self.cells[neighbor] {
                                Cell::Open => {}
                                Cell::Trees => has_trees = true,
                                Cell::Lumberyard => has_lumberyard = true,
                            }
                        }
                        if has_trees && has_lumberyard {
                            Cell::Lumberyard
                        } else {
                            Cell::Open
                        }
                    }
                }
            }
        }
    }

    fn resource_value(&self) -> (usize, usize) {
        let mut trees_count = 0;
        let mut lumberyards_count = 0;
        for x in 0..self.cells.width() {
            let column = &self.cells[x];
            for y in 0..self.cells.height() {
                match column[y] {
                    Cell::Open => {},
                    Cell::Trees => trees_count += 1,
                    Cell::Lumberyard => lumberyards_count += 1,
                }
            }
        }

        (trees_count, lumberyards_count)
    }

    fn resource_value_str(&self) -> String {
        let (trees_count, lumberyards_count) = self.resource_value();
        format!("{} * {} = {}", trees_count, lumberyards_count, trees_count * lumberyards_count)
    }
}

fn part1(input: &str) -> Result<String> {
    let mut fore_area = CollectionArea::from_str(input)?;
    let mut back_area = fore_area.clone();
    
    for _ in 0..10 {
        fore_area.update_into(&mut back_area);
        std::mem::swap(&mut fore_area, &mut back_area);
    }

    Ok(fore_area.resource_value_str())
}

fn part2(input: &str) -> Result<String> {
    let mut fore_area = CollectionArea::from_str(input)?;
    let mut back_area = fore_area.clone();

    let mut previous_states = HashMap::new();
    previous_states.insert(fore_area.clone(), 0);
    
    const ITERATIONS: usize = 1_000_000_000;
    for update_index in 1..=ITERATIONS {
        fore_area.update_into(&mut back_area);
        std::mem::swap(&mut fore_area, &mut back_area);

        if let Some(previous_index) = previous_states.insert(fore_area.clone(), update_index) {
            println!("{} == {}", previous_index, update_index);
            let cycle_length = update_index - previous_index;
            let offset = (ITERATIONS - previous_index) % cycle_length;
            if offset == 0 {
                return Ok(fore_area.resource_value_str());
            }
            let goal_index = previous_index + offset;
            let result_state = previous_states.into_iter().filter_map(|(key, value)| {
                if value == goal_index {
                    Some(key)
                }
                else {
                    None
                }
            }).next().unwrap();
            
            return Ok(result_state.resource_value_str())
        }
    }

    Ok(fore_area.resource_value_str())
}

#[test]
fn day18_test() {
    const EXAMPLE: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    assert_eq!(
        CollectionArea::from_str(EXAMPLE).unwrap().to_string(),
        EXAMPLE
    );

    assert_results!(part1, EXAMPLE => "37 * 31 = 1147");
}
