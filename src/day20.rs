day!(
    day20,
    "https://adventofcode.com/2018/day/20/input",
    part1,
    part2
);

use crate::vec2::AabbIteratorEx;
use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

type Vec2 = crate::vec2::Vec2i;

bitflags! {
    struct Room: u8 {
        const W = 0b0001;
        const E = 0b0010;
        const N = 0b0100;
        const S = 0b1000;

        const None = 0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Layout {
    rooms: HashMap<Vec2, Room>,
}

impl FromStr for Layout {
    type Err = Error;
    fn from_str(s: &str) -> Result<Layout> {
        let mut rooms = HashMap::new();
        rooms.insert(Vec2::new(0, 0), Room::None);

        if s.len() < 2 {
            return Err(Error::Input("expected some input"));
        }

        let mut chars = s.chars().skip(1).take(s.len() - 2);
        fn apply(
            chars: &mut impl Iterator<Item = char>,
            rooms: &mut HashMap<Vec2, Room>,
            start_pos: Vec2,
        ) -> Result<()> {
            let mut pos = start_pos;
            while let Some(c) = chars.next() {
                match c {
                    '|' => {
                        pos = start_pos;
                        continue;
                    }
                    ')' => return Ok(()),
                    '(' => {
                        apply(chars, rooms, pos)?;
                        continue;
                    }
                    _ => {}
                }
                let current_room = rooms.get_mut(&pos).unwrap();
                let new_room;
                let offset = #[rustfmt::skip] match c {
                    'W' => { new_room = Room::E; *current_room |= Room::W; Vec2::new(-1, 0) },
                    'E' => { new_room = Room::W; *current_room |= Room::E; Vec2::new( 1, 0) },
                    'N' => { new_room = Room::S; *current_room |= Room::N; Vec2::new(0, -1) },
                    'S' => { new_room = Room::N; *current_room |= Room::S; Vec2::new(0,  1) },
                    _ => return Err(Error::Input("invalid input character")),
                };
                std::mem::drop(current_room);
                pos += offset;
                *rooms.entry(pos).or_insert(Room::None) |= new_room;
            }
            Ok(())
        }
        apply(&mut chars, &mut rooms, Vec2::new(0, 0))?;

        Ok(Layout { rooms })
    }
}
impl Display for Layout {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (min, max) = self.rooms.keys().cloned().aabb().unwrap();
        let mut char_map = HashMap::with_capacity(self.rooms.len() * 5);
        #[rustfmt::skip] for (&position, room) in &self.rooms {
            let position = position - min;
            let char_position = (position * 2) + Vec2::new(1, 1);

            // Center
            char_map.insert(char_position, b'.');
            // Corners
            char_map.insert(char_position + Vec2::new(-1, -1), b'#');
            char_map.insert(char_position + Vec2::new( 1, -1), b'#');
            char_map.insert(char_position + Vec2::new(-1,  1), b'#');
            char_map.insert(char_position + Vec2::new( 1,  1), b'#');
            // Doors
            char_map.insert(char_position + Vec2::new(-1,  0), if room.contains(Room::W) { b'|' } else { b'#' });
            char_map.insert(char_position + Vec2::new( 1,  0), if room.contains(Room::E) { b'|' } else { b'#' });
            char_map.insert(char_position + Vec2::new( 0, -1), if room.contains(Room::N) { b'-' } else { b'#' });
            char_map.insert(char_position + Vec2::new( 0,  1), if room.contains(Room::S) { b'-' } else { b'#' });
        }
        // Finally, insert the starting position
        char_map.insert(min * -2 + Vec2::new(1, 1), b'X');

        let size = max - min + Vec2::new(1, 1);
        let size = size * 2 + Vec2::new(1, 1);
        let mut string_data = vec![b' '; ((size.x + 1) * size.y) as usize];
        for y in 0..size.y {
            string_data[((size.x + 1) * y + size.x) as usize] = b'\n';
        }
        for (p, v) in char_map {
            string_data[((size.x + 1) * p.y + p.x) as usize] = v;
        }
        string_data.pop();

        <str as Display>::fmt(&std::str::from_utf8(&string_data).unwrap(), f)
    }
}

fn calculate_costs(layout: &Layout) -> HashMap<Vec2, u32> {
    // Breadth first search to determine the costs of all rooms

    // Costs both holds the output, and is used to prevent visiting
    // the same room multiple times.
    let mut costs = HashMap::new();
    let mut visit_queue = VecDeque::new();

    visit_queue.push_back((Vec2::new(0, 0), 0));

    #[rustfmt::skip]
    while let Some((position, cost)) = visit_queue.pop_front() {
        // Don't visit cells multiple times
        match costs.entry(position) {
            Entry::Vacant(v) => v.insert(cost),
            Entry::Occupied(_) => continue,
        };

        let new_cost = cost + 1;
        let room = layout.rooms[&position];
        if room.contains(Room::W) { visit_queue.push_back((Vec2::new(position.x - 1, position.y), new_cost)); }
        if room.contains(Room::E) { visit_queue.push_back((Vec2::new(position.x + 1, position.y), new_cost)); }
        if room.contains(Room::N) { visit_queue.push_back((Vec2::new(position.x, position.y - 1), new_cost)); }
        if room.contains(Room::S) { visit_queue.push_back((Vec2::new(position.x, position.y + 1), new_cost)); }
    }

    costs
}

fn part1(input: &str) -> Result<u32> {
    let layout = Layout::from_str(input)?;
    let costs = calculate_costs(&layout);
    Ok(costs.values().cloned().max().unwrap())
}

fn part2(input: &str) -> Result<usize> {
    let layout = Layout::from_str(input)?;
    let costs = calculate_costs(&layout);
    Ok(costs.values().filter(|&&x| x >= 1000).count())
}

#[test]
fn day20_test() {
    assert_eq!(
        &Layout::from_str("^ENWWW$").unwrap().to_string(),
        "\
#########
#.|.|.|.#
#######-#
    #X|.#
    #####"
    );
    assert_eq!(
        &Layout::from_str("^ENWWW(NEEE|SSE(EE|N))$")
            .unwrap()
            .to_string(),
        "\
#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########"
    );
    assert_eq!(
        &Layout::from_str("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$")
            .unwrap()
            .to_string(),
        "\
###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########"
    );

    assert_results!(part1,
        "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"                         => 18,
        "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"               => 23,
        "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$" => 31,
    );
}
