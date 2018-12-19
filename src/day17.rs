day!(
    day17,
    "https://adventofcode.com/2018/day/17/input",
    part1,
    part2
);

use regex::Regex;
use std::fmt::{self, Display, Formatter};
use std::mem::drop;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::usize;

type Vec2 = crate::vec2::Vec2<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Sand,
    WetSand,
    Clay,
    Water,
}

impl Cell {
    #[inline(always)]
    fn is_obstacle(&self) -> bool {
        match *self {
            Cell::Sand | Cell::WetSand => false,
            Cell::Clay | Cell::Water => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Layout {
    cells: Vec<Vec<Cell>>,
    height_range: RangeInclusive<usize>,
}

impl FromStr for Layout {
    type Err = Error;
    fn from_str(s: &str) -> Result<Layout> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?m)^(x|y)=(\d+), (x|y)=(\d+)..(\d+)$").unwrap();
        }
        let mut layout = Layout {
            cells: Vec::new(),
            height_range: 0..=0,
        };
        let mut ymin: Option<usize> = None;
        let mut ymax: Option<usize> = None;
        for c in RE.captures_iter(s) {
            let is_a_x = &c[1] == "x";
            let a = c[2].parse()?;
            let lower = c[4].parse()?;
            let upper = c[5].parse()?;
            for b in lower..=upper {
                let (x, y) = if is_a_x { (a, b) } else { (b, a) };
                ymin = Some(ymin.unwrap_or(y).min(y));
                ymax = Some(ymax.unwrap_or(y).max(y));
                *layout.get_mut(Vec2::new(x, y)) = Cell::Clay;
            }
        }
        if ymin.is_none() {
            return Err(Error::Input("expected some input"));
        }
        layout.height_range = ymin.unwrap()..=ymax.unwrap();
        Ok(layout)
    }
}
impl Display for Layout {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut xmin = usize::MAX;
        let mut xmax = usize::MIN;
        for y in self.height_range.clone() {
            for (x, c) in self.cells[y].iter().enumerate() {
                if *c != Cell::Sand {
                    xmin = xmin.min(x);
                    xmax = xmax.max(x);
                }
            }
        }
        xmin -= 1;
        xmax += 1;
        let width_range = xmin..=xmax;
        let width = xmax + 1 - xmin;
        let height = 1 + *self.height_range.end() - *self.height_range.start();
        let mut s = String::with_capacity(((width + 1) * height) as usize);
        for y in self.height_range.clone() {
            for x in width_range.clone() {
                s.push(match *self.get(Vec2::new(x, y)) {
                    Cell::Sand => '.',
                    Cell::WetSand => '|',
                    Cell::Clay => '#',
                    Cell::Water => '~',
                });
            }
            s.push('\n');
        }
        s.pop();

        <String as Display>::fmt(&s, f)
    }
}
impl Layout {
    fn get(&self, position: Vec2) -> &Cell {
        if position.y >= self.cells.len() {
            return &Cell::Sand;
        }
        let row = &self.cells[position.y];
        if position.x >= row.len() {
            return &Cell::Sand;
        }
        &row[position.x]
    }
    fn get_mut(&mut self, position: Vec2) -> &mut Cell {
        while position.y >= self.cells.len() {
            self.cells.push(Vec::new());
        }
        let row = &mut self.cells[position.y];
        while position.x >= row.len() {
            row.push(Cell::Sand);
        }
        &mut row[position.x]
    }

    fn flood_from(&mut self, mut position: Vec2) -> Result<()> {
        #[derive(Debug, Clone)]
        struct FloodColumn {
            position: Vec2,
            height: usize,
        }

        position.y = position.y.max(*self.height_range.start());
        if position.y > *self.height_range.end() || self.get(position).is_obstacle() {
            return Err(Error::Input("invalid flooding point"));
        }

        let mut flood_columns = Vec::new();
        flood_columns.push(FloodColumn {
            position,
            height: 1,
        });

        'outer: loop {
            let position = if let Some(flood_column) = flood_columns.last() {
                flood_column.position + Vec2::new(0, flood_column.height - 1)
            } else {
                break 'outer;
            };

            // If the flood column is now an obstacle move up.
            // If the column has been consumed, go to the previous column.
            if self.get(position).is_obstacle() {
                let last_column = flood_columns.last_mut().unwrap();
                let new_height = last_column.height - 1;
                last_column.height = new_height;
                drop(last_column);
                if new_height == 0 {
                    flood_columns.pop();
                }
                continue 'outer;
            }

            let cell_below = *self.get(position.with_y(position.y + 1));
            if !cell_below.is_obstacle() {
                *self.get_mut(position) = Cell::WetSand;

                if position.y + 1 > *self.height_range.end() {
                    // Out of range, drop the current flood column
                    flood_columns.pop();
                } else {
                    flood_columns.last_mut().unwrap().height += 1;
                }

                continue 'outer;
            }

            // Sitting on top of an obstacle means that the water should start spreading
            // Create a range from left to right, until it hits a wall, or is above a non-obstacle
            let mut xmin = position.x;
            let mut xmax = position.x;
            let mut can_settle_left = true;
            let mut can_settle_right = true;

            loop {
                let peek_pos = position.with_x(xmin - 1);
                if self.get(peek_pos).is_obstacle() {
                    break;
                }
                xmin -= 1;
                if !self.get(peek_pos.with_y(position.y + 1)).is_obstacle() {
                    can_settle_left = false;
                    break;
                }
            }
            loop {
                let peek_pos = position.with_x(xmax + 1);
                if self.get(peek_pos).is_obstacle() {
                    break;
                }
                xmax += 1;
                if !self.get(peek_pos.with_y(position.y + 1)).is_obstacle() {
                    can_settle_right = false;
                    break;
                }
            }

            if can_settle_left && can_settle_right {
                // Create the water cells
                for x in xmin..=xmax {
                    let new_pos = position.with_x(x);
                    *self.get_mut(new_pos) = Cell::Water;
                }

                continue 'outer;
            }

            // If this row is already evaluated, then this column is finalized
            if (xmin..=xmax).all(|x| *self.get(position.with_x(x)) == Cell::WetSand) {
                flood_columns.pop();
                continue 'outer;
            }

            // If it cannot settle, it can still flow through the sand
            for x in xmin..=xmax {
                let new_pos = position.with_x(x);
                *self.get_mut(new_pos) = Cell::WetSand;
            }

            if !can_settle_left {
                flood_columns.push(FloodColumn {
                    position: Vec2::new(xmin, position.y + 1),
                    height: 1,
                });
            }
            if !can_settle_right {
                flood_columns.push(FloodColumn {
                    position: Vec2::new(xmax, position.y + 1),
                    height: 1,
                });
            }
        }

        Ok(())
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut layout = Layout::from_str(input)?;
    layout.flood_from(Vec2::new(500, 0))?;
    Ok(layout
        .cells
        .into_iter()
        .flat_map(|x| x.into_iter())
        .filter(|x| match x {
            Cell::Water | Cell::WetSand => true,
            Cell::Clay | Cell::Sand => false,
        })
        .count())
}

fn part2(input: &str) -> Result<usize> {
    let mut layout = Layout::from_str(input)?;
    layout.flood_from(Vec2::new(500, 0))?;
    Ok(layout
        .cells
        .into_iter()
        .flat_map(|x| x.into_iter())
        .filter(|x| match x {
            Cell::Water => true,
            Cell::WetSand | Cell::Clay | Cell::Sand => false,
        })
        .count())
}

#[test]
fn day17_test() {
    const EXAMPLE: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    let mut layout = Layout::from_str(EXAMPLE).unwrap();
    assert_eq!(
        layout.to_string(),
        "............#.
.#..#.......#.
.#..#..#......
.#..#..#......
.#.....#......
.#.....#......
.#######......
..............
..............
....#.....#...
....#.....#...
....#.....#...
....#######..."
    );

    layout.flood_from(Vec2::new(500, 0)).unwrap();
    assert_eq!(
        layout.to_string(),
        "......|.....#.
.#..#||||...#.
.#..#~~#|.....
.#..#~~#|.....
.#~~~~~#|.....
.#~~~~~#|.....
.#######|.....
........|.....
...|||||||||..
...|#~~~~~#|..
...|#~~~~~#|..
...|#~~~~~#|..
...|#######|.."
    );
    assert_results!(part1, EXAMPLE => 57);
    assert_results!(part2, EXAMPLE => 29);
}
