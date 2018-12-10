day!(
    day04,
    "https://adventofcode.com/2018/day/4/input",
    part1,
    part2
);

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Record {
    date: usize,
    time: isize,
    action: Action,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Action {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct TimeRange {
    date: usize,
    from: isize,
    to: isize,
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Record> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
^\[
    (?P<y>\d{4})
    -
    (?P<m>\d{2})
    -
    (?P<d>\d{2})
    \x20
    (?P<h>\d{2})
    :
    (?P<min>\d{2})
\]
\x20
(?P<action>.+)
$"
            )
            .unwrap();
        }

        let captures = RE.captures(s).ok_or(Error::Input("invalid record"))?;
        let mut date = 10000 * captures["y"].parse::<usize>()?
            + 100 * captures["m"].parse::<usize>()?
            + captures["d"].parse::<usize>()?;
        let time = match &captures["h"] {
            "23" => {
                date += 1;
                captures["min"].parse::<isize>()? - 60
            }
            "00" => captures["min"].parse()?,
            _ => return Err(Error::Input("invalid hour in log")),
        };
        Ok(Record {
            date,
            time,
            action: captures["action"].parse()?,
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Action> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Guard #(?P<id>\d+) begins shift$").unwrap();
        }

        Ok(match s {
            "falls asleep" => Action::FallAsleep,
            "wakes up" => Action::WakeUp,
            s => Action::BeginShift(
                RE.captures(s).ok_or(Error::Input("invalid action"))?["id"].parse()?,
            ),
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Record>> {
    input
        .lines()
        .map(Record::from_str)
        .collect::<Result<Vec<Record>>>()
        .map(|mut records| {
            records.sort();
            records
        })
}

fn transform(records: &Vec<Record>) -> Result<HashMap<usize, Vec<TimeRange>>> {
    let mut map = HashMap::new();
    let mut ranges = Vec::new();
    let mut id = None;
    fn switch_id(
        map: &mut HashMap<usize, Vec<TimeRange>>,
        ranges: &mut Vec<TimeRange>,
        current_id: &mut Option<usize>,
        new_id: Option<usize>,
    ) -> Result<()> {
        match current_id.clone() {
            x if x == new_id => Ok(()),
            None => {
                if ranges.len() == 0 {
                    *current_id = new_id;
                    Ok(())
                } else {
                    Err(Error::Input(
                        "id must be provided before a guard can be put to sleep",
                    ))
                }
            }
            Some(current_id_value) => {
                let mut new_ranges = new_id
                    .and_then(|new_id| map.remove(&new_id))
                    .unwrap_or(Vec::new());
                std::mem::swap(&mut new_ranges, ranges);
                map.insert(current_id_value, new_ranges);
                *current_id = new_id;
                Ok(())
            }
        }
    }

    let mut sleep_start = None;
    for record in records {
        match record.action {
            Action::BeginShift(new_id) => {
                if sleep_start.is_some() {
                    return Err(Error::Input("cannot switch while asleep"));
                }
                switch_id(&mut map, &mut ranges, &mut id, Some(new_id))?;
            }
            Action::FallAsleep => match sleep_start {
                Some(_) => return Err(Error::Input("fall asleep whilst asleep")),
                None => sleep_start = Some(record.time),
            },
            Action::WakeUp => match sleep_start {
                Some(start_time) => {
                    if start_time >= record.time {
                        return Err(Error::Input("wake up in the same minute as fall asleep"));
                    }
                    ranges.push(TimeRange {
                        date: record.date,
                        from: start_time,
                        to: record.time,
                    });
                    sleep_start = None;
                }
                None => return Err(Error::Input("cannot wake up whilst awake")),
            },
        }
    }
    if sleep_start.is_some() {
        return Err(Error::Input("data ends whilst asleep"));
    }
    switch_id(&mut map, &mut ranges, &mut id, None)?;
    Ok(map)
}

fn calculate_most_asleep(ranges: &Vec<TimeRange>) -> (usize, usize) {
    let mut asleep_count = [0usize; 60];
    for range in ranges {
        for d in range.from..range.to {
            if d >= 0 && d < 60 {
                asleep_count[d as usize] += 1;
            }
        }
    }
    asleep_count
        .iter()
        .cloned()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .unwrap()
}

fn part1(input: String) -> Result<usize> {
    let records = parse_input(&input)?;
    let time_ranges = transform(&records)?;

    time_ranges
        .iter()
        .map(|(&id, ranges)| {
            (
                id,
                ranges
                    .iter()
                    .map(|range| (range.to - range.from) as usize)
                    .sum::<usize>(),
            )
        })
        .max_by_key(|(_, asleep_count)| *asleep_count)
        .map(|(id, _)| {
            let ranges = &time_ranges[&id];
            let (most_asleep_min, _) = calculate_most_asleep(ranges);
            id * most_asleep_min
        })
        .ok_or(Error::Input("no records"))
}

fn part2(input: String) -> Result<usize> {
    let records = parse_input(&input)?;
    let time_ranges = transform(&records)?;

    time_ranges
        .iter()
        .map(|(id, ranges)| {
            let (most_asleep_min, most_asleep_count) = calculate_most_asleep(ranges);
            (id, most_asleep_min, most_asleep_count)
        })
        .max_by_key(|(_, _, most_asleep_count)| *most_asleep_count)
        .map(|(id, most_asleep_min, _)| id * most_asleep_min)
        .ok_or(Error::Input("no records"))
}

#[test]
fn day04_test() {
    const EXAMPLE: &'static str = "[1518-11-03 00:24] falls asleep
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-01 00:55] wakes up
[1518-11-05 00:45] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:55] wakes up";

    let example = vec![
        Record {
            date: 15181101,
            time: 0,
            action: Action::BeginShift(10),
        },
        Record {
            date: 15181101,
            time: 5,
            action: Action::FallAsleep,
        },
        Record {
            date: 15181101,
            time: 25,
            action: Action::WakeUp,
        },
        Record {
            date: 15181101,
            time: 30,
            action: Action::FallAsleep,
        },
        Record {
            date: 15181101,
            time: 55,
            action: Action::WakeUp,
        },
        Record {
            date: 15181102,
            time: -2,
            action: Action::BeginShift(99),
        },
        Record {
            date: 15181102,
            time: 40,
            action: Action::FallAsleep,
        },
        Record {
            date: 15181102,
            time: 50,
            action: Action::WakeUp,
        },
        Record {
            date: 15181103,
            time: 5,
            action: Action::BeginShift(10),
        },
        Record {
            date: 15181103,
            time: 24,
            action: Action::FallAsleep,
        },
        Record {
            date: 15181103,
            time: 29,
            action: Action::WakeUp,
        },
        Record {
            date: 15181104,
            time: 2,
            action: Action::BeginShift(99),
        },
        Record {
            date: 15181104,
            time: 36,
            action: Action::FallAsleep,
        },
        Record {
            date: 15181104,
            time: 46,
            action: Action::WakeUp,
        },
        Record {
            date: 15181105,
            time: 3,
            action: Action::BeginShift(99),
        },
        Record {
            date: 15181105,
            time: 45,
            action: Action::FallAsleep,
        },
        Record {
            date: 15181105,
            time: 55,
            action: Action::WakeUp,
        },
    ];
    assert_eq!(example, parse_input(EXAMPLE).unwrap());

    // PS: Why am I writing a unit test for one time parsing
    //     in Advent of Code? I don't even know...

    assert_results!(part1, EXAMPLE => 240);
    assert_results!(part2, EXAMPLE => 4455);
}
