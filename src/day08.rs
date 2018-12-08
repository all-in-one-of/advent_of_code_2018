day!(
    day08,
    "https://adventofcode.com/2018/day/8/input",
    part1,
    part2
);

use crate::vec2::Vec2i;
use regex::Regex;
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Node {
    node_index: usize,
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn parse_nodes(input: &str) -> Result<Node> {
    let nrs = input
        .split(' ')
        .map(|s| Ok(s.parse()?))
        .collect::<Result<Vec<_>>>()?;

    let mut node_index = 0;
    fn parse_node(iter: &mut impl Iterator<Item = usize>, node_index: &mut usize) -> Result<Node> {
        let current_index = *node_index;
        *node_index += 1;
        let child_count = iter.next().ok_or(Error::Input("unexpected end of node"))?;
        let meta_count = iter.next().ok_or(Error::Input("unexpected end of node"))?;
        Ok(Node {
            node_index: current_index,
            children: (0..child_count)
                .map(|_| parse_node(iter, node_index))
                .collect::<Result<_>>()?,
            metadata: (0..meta_count)
                .map(|_| iter.next().ok_or(Error::Input("unexpected end of node")))
                .collect::<Result<_>>()?,
        })
    }

    parse_node(&mut nrs.into_iter(), &mut node_index)
}

fn part1(input: String) -> Result<usize> {
    let root = parse_nodes(&input)?;
    fn visit(node: &Node) -> usize {
        node.metadata.iter().cloned().sum::<usize>()
            + node.children.iter().map(visit).sum::<usize>()
    }

    Ok(visit(&root))
}

fn part2(input: String) -> Result<usize> {
    let root = parse_nodes(&input)?;

    fn visit(node: &Node) -> usize {
        if node.children.len() == 0 {
            node.metadata.iter().cloned().sum::<usize>()
        } else {
            node.metadata
                .iter()
                .filter(|&&idx| idx != 0)
                .filter_map(|&idx| node.children.get(idx - 1).map(visit))
                .sum::<usize>()
        }
    }

    Ok(visit(&root))
}

#[test]
fn day08_test() {
    assert_results!(part1, "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2" => 138);
    assert_results!(part2, "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2" => 66);
}
