day!(
    day15,
    "https://adventofcode.com/2018/day/15/input",
    part1,
    part2
);

use crate::mat2::Mat2;
use crate::vec2::Vec2us;
use pathfinding::directed::astar::astar;
use smallvec::SmallVec;
use std::cmp::{Eq, Ordering, PartialEq};
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Wall,
    Occupied { unit: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Unit {
    position: Vec2us,
    kind: UnitKind,
    health: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnitKind {
    Elf,
    Goblin,
}

#[derive(Clone)]
struct Grid {
    cells: Mat2<Cell>,
    units: Vec<Unit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveResult {
    InRange,
    NoTargets,
    Moved(Vec2us),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttackResult {
    Nothing,
    Attacked,
    Killed,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::fmt::Write;
        let mut s = String::with_capacity(
            (self.cells.width() + 1) * self.cells.height() + self.units.len() * 9,
        );
        let mut units = String::with_capacity(30);
        for y in 0..self.cells.height() {
            units.clear();
            for x in 0..self.cells.width() {
                s.push(match self.cells[x][y] {
                    Cell::Free => '.',
                    Cell::Wall => '#',
                    Cell::Occupied { unit } => {
                        if units.is_empty() {
                            units.push_str("   ");
                        } else {
                            units.push_str(", ");
                        }
                        let kind_char = match self.units[unit].kind {
                            UnitKind::Elf => 'E',
                            UnitKind::Goblin => 'G',
                        };
                        units.push(kind_char);
                        units.push('(');
                        write!(units, "{}", self.units[unit].health).unwrap();
                        units.push(')');
                        kind_char
                    }
                });
            }
            if !units.is_empty() {
                s.push_str(&units);
            }
            s.push('\n');
        }
        s.pop();

        <String as Display>::fmt(&s, f)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <Grid as Display>::fmt(self, f)
    }
}

impl FromStr for Grid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Grid> {
        let size = {
            let mut lines = s.lines();
            let width = lines.next().ok_or(Error::Input("empty input"))?.len();
            if !lines.all(|x| x.len() == width) {
                return Err(Error::Input("inconsistent line width"));
            }
            let height = s.lines().count();
            Vec2us::new(width, height)
        };

        let mut cells = Mat2::new(Cell::Free, size);
        let mut units = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells[x][y] = match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Free,
                    c if c == 'G' || c == 'E' => {
                        units.push(Unit {
                            position: Vec2us::new(x, y),
                            kind: if c == 'E' {
                                UnitKind::Elf
                            } else {
                                UnitKind::Goblin
                            },
                            health: 200,
                        });
                        Cell::Occupied {
                            unit: units.len() - 1,
                        }
                    }
                    _ => {
                        return Err(Error::Input("invalid character in input"));
                    }
                };
            }
        }

        Ok(Grid { cells, units })
    }
}

impl Eq for Grid {}
impl PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        if self.cells.size() != other.cells.size() {
            return false;
        }

        for x in 0..self.cells.width() {
            for y in 0..self.cells.height() {
                let other_cell = other.cells[x][y];
                if !match self.cells[x][y] {
                    Cell::Free => other_cell == Cell::Free,
                    Cell::Wall => other_cell == Cell::Wall,
                    Cell::Occupied { unit } => {
                        if let Cell::Occupied { unit: other_unit } = other_cell {
                            debug_assert_eq!(Vec2us::new(x, y), self.units[unit].position);
                            debug_assert_eq!(Vec2us::new(x, y), other.units[other_unit].position);
                            self.units[unit] == other.units[other_unit]
                        } else {
                            false
                        }
                    }
                } {
                    return false;
                }
            }
        }

        true
    }
}

impl Grid {
    fn swap_unit_ids(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        let unit_a = self.units[a].clone();
        let unit_b = self.units[b].clone();
        self.units.swap(a, b);

        if unit_a.health != 0 {
            self.cells[unit_a.position] = Cell::Occupied { unit: b };
        }
        if unit_b.health != 0 {
            self.cells[unit_b.position] = Cell::Occupied { unit: a };
        }
    }

    fn normalize(&mut self) {
        let mut next_unit_id = 0;
        // Move unit ids to board order
        for y in 0..self.cells.height() {
            for x in 0..self.cells.width() {
                if let Cell::Occupied { unit } = self.cells[x][y] {
                    self.swap_unit_ids(next_unit_id, unit);
                    next_unit_id += 1;
                }
            }
        }
        // All units that were killed are on the end now, remove them
        while let Some(unit) = self.units.pop() {
            if unit.health != 0 {
                self.units.push(unit);
                break;
            }
        }
    }

    fn move_unit(&mut self, unit: usize, new_position: Vec2us) {
        let unit_ref = &mut self.units[unit];
        let old_position = unit_ref.position;
        if old_position == new_position {
            return;
        }
        debug_assert!(self.cells[old_position] == Cell::Occupied { unit });
        debug_assert!(self.cells[new_position] == Cell::Free);
        unit_ref.position = new_position;
        self.cells[old_position] = Cell::Free;
        self.cells[new_position] = Cell::Occupied { unit };
    }

    fn neighbors(&self, position: Vec2us) -> impl Iterator<Item = Vec2us> {
        let size = self.cells.size();
        neighbors(position).filter(move |position| position.all(&size, |a, b| a < b))
    }

    fn free_neighbors<'a>(&'a self, position: Vec2us) -> impl Iterator<Item = Vec2us> + 'a {
        self.neighbors(position)
            .filter(move |position| self.cells[*position] == Cell::Free)
    }

    fn path_step(&self, from: Vec2us, to: Vec2us) -> Option<(u32, Vec2us)> {
        if from == to {
            return Some((0, to));
        }
        let fn_neighbors = |&position: &Vec2us| -> SmallVec<[(Vec2us, u32); 4]> {
            // Adjust the costs of traversing to neighbors of the first position
            // so that stepping through will prefer reading order over alternatives.
            if position == from {
                self.free_neighbors(position)
                    .enumerate()
                    .map(|(idx, position)| (position, idx as u32 + 1))
                    .collect()
            } else {
                self.free_neighbors(position)
                    .map(|position| (position, 5))
                    .collect()
            }
        };
        #[rustfmt::skip]
        let fn_heuristic = move |&position: &Vec2us| {
            if position == from {
                1
            } else {
                (     ((position.x as i32) - (to.x as i32)).abs()
                    + ((position.y as i32) - (to.y as i32)).abs()) as u32
                    * 5
            }
        };
        let fn_goal = move |position: &Vec2us| *position == to;

        if let Some((path, length)) = astar(&from, fn_neighbors, fn_heuristic, fn_goal) {
            let length = length / 5 + 1;
            let neighbor = path[1];
            return Some((length, neighbor));
        }
        None
    }

    fn update_unit_movement(&mut self, unit: usize) -> MoveResult {
        let unit_kind = self.units[unit].kind;
        let current_position = self.units[unit].position;

        // Don't move if there's a neighboring unit of the other kind
        for neighbor in self.neighbors(current_position) {
            if let Cell::Occupied { unit: other_unit } = self.cells[neighbor] {
                if self.units[other_unit].kind != unit_kind {
                    return MoveResult::InRange;
                }
            }
        }

        // Select all unique target cells
        let mut target_cells = self
            .units
            .iter()
            .filter(|other_unit| other_unit.health != 0 && other_unit.kind != unit_kind)
            .flat_map(|other_unit| self.free_neighbors(other_unit.position))
            .map(|position| (position, 0, position))
            .collect::<Vec<_>>();

        if target_cells.len() == 0 {
            return MoveResult::NoTargets;
        }

        target_cells.sort_by(|(a, _, _), (b, _, _)| reading_order(a, b));
        target_cells.dedup();

        // Pathfind to all targets
        target_cells.drain_filter(|(position, path_length, next_position)| {
            if let Some((l, p)) = self.path_step(current_position, *position) {
                *path_length = l;
                *next_position = p;
                false
            } else {
                true
            }
        });

        if target_cells.len() == 0 {
            return MoveResult::NoTargets;
        }

        // Keep shortest paths only
        let shortest_path_length = target_cells
            .iter()
            .map(|(_, path_length, _)| *path_length)
            .min()
            .unwrap();
        target_cells.drain_filter(|(_, path_length, _)| *path_length != shortest_path_length);

        // Break ties via reading order
        target_cells.sort_by(|(_, _, a), (_, _, b)| reading_order(a, b));

        let new_position = target_cells[0].2;
        self.move_unit(unit, new_position);

        MoveResult::Moved(new_position)
    }

    fn update_unit_attack(&mut self, unit: usize, elf_attack_power: u8) -> AttackResult {
        let unit_kind = self.units[unit].kind;
        let attack_power = if unit_kind == UnitKind::Elf {
            elf_attack_power
        } else {
            3
        };

        // Find enemy neighbors
        let mut neighbors = self
            .neighbors(self.units[unit].position)
            .filter_map(|position| {
                if let Cell::Occupied { unit: other_unit } = self.cells[position] {
                    if self.units[other_unit].kind != unit_kind {
                        return Some(other_unit);
                    }
                }
                None
            })
            .collect::<SmallVec<[usize; 4]>>();
        if neighbors.len() == 0 {
            return AttackResult::Nothing;
        }

        // Take lowest health first, break ties with reading order
        neighbors.sort_by(|a, b| {
            let a = &self.units[*a];
            let b = &self.units[*b];
            a.health
                .cmp(&b.health)
                .then_with(|| reading_order(&a.position, &b.position))
        });

        // Damage or kill the enemy
        let enemy = &mut self.units[neighbors[0]];
        if enemy.health > attack_power {
            enemy.health -= attack_power;
            AttackResult::Attacked
        } else {
            enemy.health = 0;
            self.cells[enemy.position] = Cell::Free;
            AttackResult::Killed
        }
    }

    #[cfg(test)]
    fn update_all_unit_movement(&mut self) {
        let mut units: Vec<_> = (0..self.units.len()).collect();
        units.drain_filter(|unit| self.units[*unit].health == 0);
        units.sort_by(|a, b| reading_order(&self.units[*a].position, &self.units[*b].position));
        for unit in units {
            self.update_unit_movement(unit);
        }
    }

    fn update_all_custom_power(&mut self, elf_attack_power: u8) -> (bool, bool) {
        let mut units: Vec<_> = (0..self.units.len())
            .filter(|unit| self.units[*unit].health != 0)
            .collect();
        units.sort_by(|a, b| reading_order(&self.units[*a].position, &self.units[*b].position));
        let elf_count = units
            .iter()
            .cloned()
            .filter(|unit| self.units[*unit].kind == UnitKind::Elf)
            .count();
        let goblin_count = units.len() - elf_count;
        if elf_count == 0 || goblin_count == 0 {
            return (false, false);
        }

        let mut new_elf_count = elf_count;
        let mut new_goblin_count = goblin_count;

        let mut any_killed = false;

        for unit in units {
            // Unit may have died in the meanwhile
            if self.units[unit].health == 0 {
                continue;
            }
            let opposite_count = match self.units[unit].kind {
                UnitKind::Elf => new_goblin_count,
                UnitKind::Goblin => new_elf_count,
            };
            if opposite_count == 0 {
                self.normalize();
                return (false, new_elf_count != elf_count);
            }
            self.update_unit_movement(unit);
            if self.update_unit_attack(unit, elf_attack_power) == AttackResult::Killed {
                any_killed = true;
                match self.units[unit].kind {
                    UnitKind::Elf => new_goblin_count -= 1,
                    UnitKind::Goblin => new_elf_count -= 1,
                }
            }
        }

        if any_killed {
            self.normalize();
            return (true, new_elf_count != elf_count);
        }

        (true, false)
    }

    fn update_all(&mut self) -> bool {
        self.update_all_custom_power(3).0
    }
}

fn neighbors(position: Vec2us) -> impl Iterator<Item = Vec2us> {
    (0..4).filter_map(move |step| match step {
        // The order here is significant, being in reading order
        // so first evaluate up, then left, then right, then down.
        0 if position.y > 0 => Some(Vec2us::new(position.x, position.y - 1)),
        1 if position.x > 0 => Some(Vec2us::new(position.x - 1, position.y)),
        2 => Some(Vec2us::new(position.x + 1, position.y)),
        3 => Some(Vec2us::new(position.x, position.y + 1)),
        _ => None,
    })
}

fn reading_order(a: &Vec2us, b: &Vec2us) -> Ordering {
    a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x))
}

fn part1(input: &str) -> Result<String> {
    let mut grid: Grid = input.parse()?;
    let mut round_count = 0;
    while grid.update_all() {
        round_count += 1;
    }

    let total_health: u32 = grid.units.iter().map(|unit| unit.health as u32).sum();

    Ok(format!(
        "{} * {} = {}",
        round_count,
        total_health,
        round_count * total_health
    ))
}

fn part2(input: &str) -> Result<String> {
    let grid: Grid = input.parse()?;

    // Inclusive bounds for a binary search over attack power
    let mut min_ap = 3;
    let mut max_ap = 100;

    let (attack_power, grid, round_count) = loop {
        let attack_power = (max_ap - min_ap) / 2 + min_ap;

        let mut grid = grid.clone();
        let mut round_count = 0;
        let victory = loop {
            let (any_updates, any_elf_killed) = grid.update_all_custom_power(attack_power);
            if any_elf_killed {
                break false;
            }
            if !any_updates {
                break true;
            }
            round_count += 1;
        };

        if !victory {
            // Didn't win, therefore should try again with a higher buff
            min_ap = attack_power + 1;
            if min_ap > max_ap {
                return Err(Error::Input("cannot solve with maximum attack power"));
            }
            continue;
        }

        // Won with minimum amount of buff
        if attack_power == min_ap {
            break (attack_power, grid, round_count);
        }
        // Attempt winning with less power
        max_ap = attack_power;
    };

    let total_health: u32 = grid.units.iter().map(|unit| unit.health as u32).sum();
    Ok(format!(
        "{} * {} = {} ({} attack power)",
        round_count,
        total_health,
        round_count * total_health,
        attack_power,
    ))
}

#[test]
fn day15_test() {
    // Movement example
    let mut movement: Grid = "#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"
        .parse()
        .unwrap();
    movement.update_all_unit_movement();
    assert_eq!(
        Grid::from_str(
            "#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########"
        )
        .unwrap(),
        movement
    );
    movement.update_all_unit_movement();
    assert_eq!(
        Grid::from_str(
            "#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########"
        )
        .unwrap(),
        movement
    );
    movement.update_all_unit_movement();
    assert_eq!(
        Grid::from_str(
            "#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"
        )
        .unwrap(),
        movement
    );

    // Fighting and movement sample
    let mut attack: Grid = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
        .parse()
        .unwrap();

    // 1 round
    assert!(attack.update_all());
    let mut state: Grid = "#######
#..G..#
#...EG#
#.#G#G#
#...#E#
#.....#
#######"
        .parse()
        .unwrap();
    state.units[1].health = 197;
    state.units[2].health = 197;
    state.units[4].health = 197;
    state.units[5].health = 197;
    assert_eq!(attack, state);

    // 2 rounds
    assert!(attack.update_all());
    let mut state: Grid = "#######
#...G.#
#..GEG#
#.#.#G#
#...#E#
#.....#
#######"
        .parse()
        .unwrap();
    state.units[2].health = 188;
    state.units[3].health = 194;
    state.units[4].health = 194;
    state.units[5].health = 194;
    assert_eq!(attack, state);

    // 23 rounds
    for _ in 0..23 - 2 {
        assert!(attack.update_all());
    }
    let mut state: Grid = "#######
#...G.#
#..G.G#
#.#.#G#
#...#E#
#.....#
#######"
        .parse()
        .unwrap();
    state.units[2].health = 131;
    state.units[3].health = 131;
    state.units[4].health = 131;
    assert_eq!(attack, state);

    // 24 rounds
    assert!(attack.update_all());
    let mut state: Grid = "#######
#..G..#
#...G.#
#.#G#G#
#...#E#
#.....#
#######"
        .parse()
        .unwrap();
    state.units[1].health = 131;
    state.units[3].health = 128;
    state.units[4].health = 128;
    assert_eq!(attack, state);

    // 25 rounds
    assert!(attack.update_all());
    let mut state: Grid = "#######
#.G...#
#..G..#
#.#.#G#
#..G#E#
#.....#
#######"
        .parse()
        .unwrap();
    state.units[1].health = 131;
    state.units[2].health = 125;
    state.units[4].health = 125;
    assert_eq!(attack, state);

    // 26 rounds
    assert!(attack.update_all());
    let mut state: Grid = "#######
#G....#
#.G...#
#.#.#G#
#...#E#
#..G..#
#######"
        .parse()
        .unwrap();
    state.units[1].health = 131;
    state.units[2].health = 122;
    state.units[3].health = 122;
    assert_eq!(attack, state);

    // 27 rounds
    assert!(attack.update_all());
    let mut state: Grid = "#######
#G....#
#.G...#
#.#.#G#
#...#E#
#...G.#
#######"
        .parse()
        .unwrap();
    state.units[1].health = 131;
    state.units[2].health = 119;
    state.units[3].health = 119;
    assert_eq!(attack, state);

    // 28 rounds
    assert!(attack.update_all());
    let mut state: Grid = "#######
#G....#
#.G...#
#.#.#G#
#...#E#
#....G#
#######"
        .parse()
        .unwrap();
    state.units[1].health = 131;
    state.units[2].health = 116;
    state.units[3].health = 113;
    assert_eq!(attack, state);

    assert_results!(part1,
"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######" => "47 * 590 = 27730",
"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######" => "37 * 982 = 36334",
"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######" => "46 * 859 = 39514",
"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######" => "35 * 793 = 27755",
"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######" => "54 * 536 = 28944",
"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########" => "20 * 937 = 18740");

    assert_results!(part2,
"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######" => "29 * 172 = 4988 (15 attack power)",
"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######" => "33 * 948 = 31284 (4 attack power)",
"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######" => "37 * 94 = 3478 (15 attack power)",
"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######" => "39 * 166 = 6474 (12 attack power)",
"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########" => "30 * 38 = 1140 (34 attack power)"
);
}
