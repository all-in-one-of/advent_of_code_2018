day!(
    day07,
    "https://adventofcode.com/2018/day/7/input",
    part1,
    part2
);

use regex::Regex;
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Rule {
    id: char,
    dep: char,
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Rule> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Step (?P<dep>[A-Z]) must be finished before step (?P<id>[A-Z]) can begin\.$"
            )
            .unwrap();
        };
        RE.captures(s)
            .ok_or(Error::Input("missing input"))
            .map(|c| Rule {
                id: c["id"].as_bytes()[0] as char,
                dep: c["dep"].as_bytes()[0] as char,
            })
    }
}

fn create_dependency_graph(input: &str) -> Result<HashMap<char, Vec<char>>> {
    let rules = input
        .lines()
        .map(Rule::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut dependencies: HashMap<char, Vec<char>> = HashMap::new();
    for rule in rules {
        dependencies.entry(rule.id).or_default().push(rule.dep);
        dependencies.entry(rule.dep).or_default();
    }
    Ok(dependencies)
}

fn part1(input: &str) -> Result<String> {
    let mut dependencies = create_dependency_graph(input)?;

    let mut order = String::with_capacity(dependencies.len());
    while !dependencies.is_empty() {
        let id = dependencies
            .iter()
            .filter(|(_, deps)| deps.iter().all(|dep| !dependencies.contains_key(dep)))
            .map(|(&id, _)| id)
            .min()
            .ok_or(Error::Input("unsolveable graph"))?;
        dependencies.remove(&id);
        order.push(id);
    }

    Ok(order)
}

fn part2_impl(input: &str, extra_time: usize, worker_count: usize) -> Result<usize> {
    let mut dependencies = create_dependency_graph(input)?;
    let mut in_progress = HashSet::new();

    #[derive(Debug, Clone)]
    enum WorkerState {
        Idle,
        Working { id: char, time_left: usize },
    }

    let mut workers = vec![WorkerState::Idle; worker_count];
    let mut time = 0;
    while !in_progress.is_empty() || !dependencies.is_empty() {
        // Assign new tasks to workers
        let mut available_steps = dependencies
            .iter()
            .filter(|(id, _)| !in_progress.contains(*id))
            .filter(|(_, deps)| deps.iter().all(|dep| !dependencies.contains_key(dep)))
            .map(|(&id, _)| id)
            .collect::<SmallVec<[char; 16]>>();
        available_steps.sort();
        'next_step: for available_step in available_steps {
            for worker in &mut workers {
                if let &mut WorkerState::Idle = worker {
                    in_progress.insert(available_step);
                    *worker = WorkerState::Working {
                        id: available_step,
                        time_left: (available_step as usize) - ('A' as usize) + 1 + extra_time,
                    };
                    continue 'next_step;
                }
            }
            // No worker was found for this step, so there's no point
            // in trying for other steps.
            break;
        }

        // Progress time
        time += 1;
        for worker in &mut workers {
            if let WorkerState::Working { id, time_left } = worker {
                *time_left -= 1;
                if *time_left == 0 {
                    in_progress.remove(&*id);
                    dependencies.remove(&*id);
                    std::mem::drop(id);
                    std::mem::drop(time_left);
                    *worker = WorkerState::Idle;
                }
            }
        }
    }

    Ok(time)
}

fn part2(input: &str) -> Result<usize> {
    part2_impl(input, 60, 5)
}

#[test]
fn day07_test() {
    const EXAMPLE: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    fn part2_test(input: &str) -> Result<usize> {
        part2_impl(input, 0, 2)
    }

    assert_results!(part1, EXAMPLE => "CABDFE");
    assert_results!(part2_test, EXAMPLE => 15);
}
