day!(
    day13,
    "https://adventofcode.com/2018/day/13/input",
    part1,
    part2
);

use crate::vec2::Vec2us;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Vertical,     // |
    Horizontal,   // -
    CornerCW,     // /  clockwise when approaching from north or south
    CornerCCW,    // \  counter clockwise when approaching from north or south
    Intersection, // +
    Crash,        // X
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    XPos,
    XNeg,
    YPos,
    YNeg,
}

#[derive(Debug, Clone)]
struct Cart {
    position: Vec2us,
    direction: Direction,
    intersections_taken: usize,
}

#[derive(Debug, Clone)]
struct Board {
    layout: HashMap<Vec2us, Cell>,
    carts: Vec<Cart>,
}

impl TryFrom<char> for Cell {
    type Error = Error;
    fn try_from(c: char) -> Result<Cell> {
        Ok(match c {
            '|' => Cell::Vertical,
            '-' => Cell::Horizontal,
            '/' => Cell::CornerCW,
            '\\' => Cell::CornerCCW,
            '+' => Cell::Intersection,
            'X' => Cell::Crash,
            _ => return Err(Error::Input("invalid character")),
        })
    }
}

impl From<Cell> for char {
    fn from(c: Cell) -> char {
        match c {
            Cell::Vertical => '|',
            Cell::Horizontal => '-',
            Cell::CornerCW => '/',
            Cell::CornerCCW => '\\',
            Cell::Intersection => '+',
            Cell::Crash => 'X',
        }
    }
}

impl FromStr for Board {
    type Err = Error;
    fn from_str(s: &str) -> Result<Board> {
        let mut layout = HashMap::new();
        let mut carts = Vec::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Vec2us::new(x, y);
                let (direction, cell) = match c {
                    '<' => (Direction::XNeg, Cell::Horizontal),
                    '>' => (Direction::XPos, Cell::Horizontal),
                    '^' => (Direction::YNeg, Cell::Vertical),
                    'v' => (Direction::YPos, Cell::Vertical),
                    ' ' => continue,
                    _ => {
                        layout.insert(position, Cell::try_from(c)?);
                        continue;
                    }
                };
                layout.insert(position, cell);
                carts.push(Cart {
                    position,
                    direction,
                    intersections_taken: 0,
                });
            }
        }

        Ok(Board { layout, carts })
    }
}

impl Board {
    fn tick(&mut self) -> SmallVec<[Vec2us; 4]> {
        let mut new_crashes = SmallVec::new();

        // Sort the carts based on which ones need to be evaluated/updated first
        self.carts.sort_by(|a, b| {
            a.position
                .y
                .cmp(&b.position.y)
                .then_with(|| a.position.x.cmp(&b.position.x))
        });

        let mut cart_index = 0;
        while cart_index < self.carts.len() {
            let cart = &mut self.carts[cart_index];
            cart_index += 1;

            // Move
            match cart.direction {
                Direction::XNeg => cart.position.x -= 1,
                Direction::XPos => cart.position.x += 1,
                Direction::YNeg => cart.position.y -= 1,
                Direction::YPos => cart.position.y += 1,
            }

            // Check for intersections and corners
            match self.layout[&cart.position] {
                Cell::CornerCW => {
                    // corner /
                    cart.direction = match cart.direction {
                        Direction::XNeg => Direction::YPos,
                        Direction::XPos => Direction::YNeg,
                        Direction::YNeg => Direction::XPos,
                        Direction::YPos => Direction::XNeg,
                    };
                }
                Cell::CornerCCW => {
                    // corner \
                    cart.direction = match cart.direction {
                        Direction::XNeg => Direction::YNeg,
                        Direction::XPos => Direction::YPos,
                        Direction::YNeg => Direction::XNeg,
                        Direction::YPos => Direction::XPos,
                    };
                }
                Cell::Intersection => {
                    match cart.intersections_taken % 3 {
                        0 => {
                            // turn left
                            cart.direction = match cart.direction {
                                Direction::XNeg => Direction::YPos,
                                Direction::XPos => Direction::YNeg,
                                Direction::YNeg => Direction::XNeg,
                                Direction::YPos => Direction::XPos,
                            };
                        }
                        1 => {} // straight
                        2 => {
                            // turn right
                            cart.direction = match cart.direction {
                                Direction::XNeg => Direction::YNeg,
                                Direction::XPos => Direction::YPos,
                                Direction::YNeg => Direction::XPos,
                                Direction::YPos => Direction::XNeg,
                            };
                        }
                        _ => unreachable!(),
                    }
                    cart.intersections_taken += 1;
                }
                _ => {}
            }

            let position = cart.position;
            std::mem::drop(cart);

            // Check for collisions, and remove them if found
            // could use some cleaning up...
            let mut crashing_carts = self
                .carts
                .iter()
                .enumerate()
                .filter(|(_, cart)| cart.position == position)
                .map(|(idx, _)| idx);
            let first_cart = crashing_carts.next().unwrap();
            if let Some(second_cart) = crashing_carts.next() {
                std::mem::drop(crashing_carts);
                new_crashes.push(position);

                // Remove crashed carts
                if second_cart > first_cart {
                    self.carts.remove(second_cart);
                    self.carts.remove(first_cart);
                }
                else {
                    self.carts.remove(first_cart);
                    self.carts.remove(second_cart);
                }
                cart_index -= 1;
                if cart_index >= second_cart {
                    cart_index -= 1;
                }
            }
        }

        new_crashes
    }
}

fn part1(input: &str) -> Result<Vec2us> {
    let mut board: Board = input.parse()?;
    for _ in 0..1_000_000 {
        let new_crashes = board.tick();
        if !new_crashes.is_empty() {
            return Ok(new_crashes[0]);
        }
    }
    Err(Error::Input("cannot solve in a million iterations"))
}

fn part2(input: &str) -> Result<Vec2us> {
    let mut board: Board = input.parse()?;
    for _ in 0..1_000_000 {
        let _ = board.tick();
        if board.carts.len() == 1 {
            return Ok(board.carts[0].position);
        }
    }
    Err(Error::Input("cannot solve in a million iterations"))
}

#[test]
fn day13_test() {
    const EXAMPLE_PT1: &str = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

    const EXAMPLE_PT2: &str = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

    assert_results!(part1, EXAMPLE_PT1 => Vec2us::new(7, 3));
    assert_results!(part2, EXAMPLE_PT2 => Vec2us::new(6, 4));
}
