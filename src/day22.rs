day!(
    day22,
    "https://adventofcode.com/2018/day/22/input",
    part1,
    part2
);

use crate::mat2::Mat2;
use num_traits::{One, Zero};
use pathfinding::directed::astar::astar;
use regex::Regex;
use smallvec::SmallVec;
use std::fmt::{self, Display};
use std::str::FromStr;

type Vec2 = crate::vec2::Vec2us;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Input {
    depth: u32,
    target: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

type Layout = Mat2<RegionType>;
#[derive(Debug, Clone, PartialEq, Eq)]
struct CaveSystem {
    depth: u32,
    target: Vec2,
    layout: Layout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

impl From<u32> for RegionType {
    fn from(v: u32) -> RegionType {
        match v % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }
}
impl From<RegionType> for u32 {
    fn from(v: RegionType) -> u32 {
        match v {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }
}
impl From<RegionType> for char {
    fn from(v: RegionType) -> char {
        match v {
            RegionType::Rocky => '.',
            RegionType::Wet => '=',
            RegionType::Narrow => '|',
        }
    }
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Input> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^depth: (\d+)\ntarget: (\d+),(\d+)$").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or(Error::Input("invalid input structure"))?;
        Ok(Input {
            depth: c[1].parse()?,
            target: Vec2::new(c[2].parse()?, c[3].parse()?),
        })
    }
}

impl Display for CaveSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.layout.size();
        let mut s = String::with_capacity((size.x + 1) * size.y);
        for y in 0..size.y {
            for x in 0..size.x {
                if x == self.target.x && y == self.target.y {
                    s.push('T');
                } else {
                    s.push(self.layout[x][y].into());
                }
            }
            s.push('\n');
        }
        // Valid because only ASCII characters are pushed
        unsafe {
            s.as_bytes_mut()[0] = b'M';
        }
        s.pop();

        <String as Display>::fmt(&s, f)
    }
}

impl CaveSystem {
    fn new(size: Vec2, input: &Input) -> CaveSystem {
        let mut erosion_levels = Mat2::new(0u32, size);

        for x in 0..size.x {
            erosion_levels[x][0] = (x as u32 * 16807 + input.depth) % 20183;
        }
        for y in 1..size.y {
            erosion_levels[0][y] = (y as u32 * 48271 + input.depth) % 20183;
        }

        for x in 1..size.x {
            for y in 1..size.y {
                if x == input.target.x && y == input.target.y {
                    erosion_levels[x][y] = input.depth % 20183;
                    continue;
                }
                erosion_levels[x][y] =
                    (erosion_levels[x - 1][y] * erosion_levels[x][y - 1] + input.depth) % 20183;
            }
        }

        let mut layout = Layout::new(RegionType::Rocky, size);
        for x in 0..size.x {
            let dst = &mut layout[x];
            let src = &erosion_levels[x];
            for y in 0..size.y {
                dst[y] = src[y].into();
            }
        }

        CaveSystem {
            depth: input.depth,
            target: input.target,
            layout,
        }
    }

    fn from_input(input: &Input) -> CaveSystem {
        CaveSystem::new(input.target + Vec2::one(), input)
    }
}

impl RegionType {
    fn available_tools(&self) -> &'static [Tool; 2] {
        #[rustfmt::skip] match *self {
            RegionType::Rocky  => &[Tool::Torch  , Tool::ClimbingGear],
            RegionType::Wet    => &[Tool::Neither, Tool::ClimbingGear],
            RegionType::Narrow => &[Tool::Neither, Tool::Torch       ],
        }
    }

    fn can_enter(&self, tool: Tool) -> bool {
        #[rustfmt::skip] match *self {
            RegionType::Rocky  => tool != Tool::Neither     ,
            RegionType::Wet    => tool != Tool::Torch       ,
            RegionType::Narrow => tool != Tool::ClimbingGear,
        }
    }
}

fn part1(input: &str) -> Result<u32> {
    let cave = CaveSystem::from_input(&input.parse()?);
    let size = cave.layout.size();
    let mut risk = 0;
    for x in 0..size.x {
        let column = &cave.layout[x];
        for y in 0..size.y {
            risk += u32::from(column[y]);
        }
    }
    Ok(risk)
}

fn part2(input: &str) -> Result<u32> {
    let input = Input::from_str(input)?;
    // 8 times the size will guarantee that the shortest path is in range
    // because if it'd need more than 8 times the size, the shortest path
    // could also just switch to a new tool every cell.
    let cave = CaveSystem::new((input.target + Vec2::one()) * 8, &input);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        pos: Vec2,
        tool: Tool,
    }

    let (_path, cost) = astar(
        &Node {
            pos: Vec2::zero(),
            tool: Tool::Torch,
        },
        |node| -> SmallVec<[(Node, u32); 5]> {
            let mut next = SmallVec::new();
            // Switch tools
            let next_tool = cave.layout[node.pos]
                .available_tools()
                .iter()
                .cloned()
                .filter(|tool| *tool != node.tool)
                .next()
                .unwrap();
            next.push((Node {
                pos: node.pos,
                tool: next_tool,
            }, 7));

            // Advance to other positions
            {
                let mut add_if_accessible = |pos: Vec2| {
                    if cave.layout[pos].can_enter(node.tool) {
                        next.push((
                            Node {
                                pos,
                                tool: node.tool,
                            },
                            1,
                        ));
                    }
                };
                if node.pos.x != 0 {
                    add_if_accessible(node.pos.with_x(node.pos.x - 1)); // left
                }
                if node.pos.x + 1 < cave.layout.width() {
                    add_if_accessible(node.pos.with_x(node.pos.x + 1)); // right
                }
                if node.pos.y != 0 {
                    add_if_accessible(node.pos.with_y(node.pos.y - 1)); // top
                }
                if node.pos.y + 1 < cave.layout.height() {
                    add_if_accessible(node.pos.with_y(node.pos.y + 1)); // bottom
                }
            }
            next
        },
        #[rustfmt::skip] |node| {
        ( // Manhathan distance
            (if node.pos.x > cave.target.x { node.pos.x - cave.target.x } else { cave.target.x - node.pos.x }) +
            (if node.pos.y > cave.target.y { node.pos.y - cave.target.y } else { cave.target.y - node.pos.y })
        ) as u32
    },
        |node| *node == Node { pos: cave.target, tool: Tool::Torch },
    ).unwrap();
    Ok(cost)
}

#[test]
fn day22_test() {
    assert_eq!(
        Input {
            depth: 8112,
            target: Vec2::new(13, 743)
        },
        "depth: 8112\ntarget: 13,743".parse().unwrap()
    );

    let example = Input {
        depth: 510,
        target: Vec2::new(10, 10),
    };
    assert_eq!(
        &CaveSystem::new(Vec2::new(16, 16), &example).to_string(),
        "\
M=.|=.|.|=.|=|=.
.|=|=|||..|.=...
.==|....||=..|==
=.|....|.==.|==.
=|..==...=.|==..
=||.=.=||=|=..|=
|.=.===|||..=..|
|..==||=.|==|===
.=..===..=|.|||.
.======|||=|=.|=
.===|=|===T===||
=|||...|==..|=.|
=.=|=.=..=.||==|
||=|=...|==.=|==
|=.=||===.|||===
||.|==.|.|.||=||"
    );

    assert_results!(part1, "depth: 510\ntarget: 10,10" => 114);
    assert_results!(part2, "depth: 510\ntarget: 10,10" => 45);
}
