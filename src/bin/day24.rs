use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending, space1, u8},
    multi::fold_many1,
    sequence::{separated_pair, terminated, tuple},
};
use petgraph::{Graph, algo::toposort, graph::NodeIndex};

fn parse_init_map(input: &str) -> nom::IResult<&str, HashMap<&str, u8>> {
    fold_many1(
        terminated(separated_pair(alphanumeric1, tag(": "), u8), line_ending),
        HashMap::new,
        |mut acc, (name, value)| {
            acc.insert(name, value);
            acc
        },
    )(input)
}

#[derive(Debug)]
enum Op<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

impl Op<'_> {
    fn result(&self, value_map: &HashMap<&str, u8>) -> u8 {
        match self {
            Op::And(l, r) => {
                let (l, r) = (value_map.get(l).unwrap(), value_map.get(r).unwrap());
                l & r
            }
            Op::Or(l, r) => {
                let (l, r) = (value_map.get(l).unwrap(), value_map.get(r).unwrap());
                l | r
            }
            Op::Xor(l, r) => {
                let (l, r) = (value_map.get(l).unwrap(), value_map.get(r).unwrap());
                l ^ r
            }
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    op: Op<'a>,
}

fn parse_dep_graph(input: &str) -> nom::IResult<&str, Graph<Node<'_>, ()>> {
    fold_many1(
        terminated(
            tuple((
                alphanumeric1,
                space1,
                alpha1,
                space1,
                alphanumeric1,
                tag(" -> "),
                alphanumeric1,
            )),
            line_ending,
        ),
        || (Graph::new(), HashMap::new(), HashMap::new()),
        |(mut graph, mut node_map, mut children), (lhs, _, op, _, rhs, _, dst)| {
            let node = Node {
                name: dst,
                op: match op {
                    "AND" => Op::And(lhs, rhs),
                    "OR" => Op::Or(lhs, rhs),
                    "XOR" => Op::Xor(lhs, rhs),
                    _ => unreachable!("skill issue"),
                },
            };

            // A XOR B -> C
            // ...
            // ...
            // C AND D -> E
            let dst_idx = *node_map.entry(dst).or_insert_with(|| graph.add_node(node));
            children
                .entry(lhs)
                .or_insert(HashSet::new())
                .insert(dst_idx);
            children
                .entry(rhs)
                .or_insert(HashSet::new())
                .insert(dst_idx);
            if let Some(&l_idx) = node_map.get(lhs) {
                graph.add_edge(l_idx, dst_idx, ());
            }
            if let Some(&r_idx) = node_map.get(rhs) {
                graph.add_edge(r_idx, dst_idx, ());
            }
            if let Some(dst_children) = children.get(dst) {
                for &dst_child in dst_children {
                    graph.add_edge(dst_idx, dst_child, ());
                }
            }

            (graph, node_map, children)
        },
    )
    .map(|(graph, _, _)| graph)
    .parse(input)
}

fn parse(input: &str) -> nom::IResult<&str, (Graph<Node<'_>, ()>, HashMap<&str, u8>)> {
    let (input, map) = parse_init_map(input)?;
    let (input, _) = line_ending(input)?;
    let (input, graph) = parse_dep_graph(input)?;
    Ok((input, (graph, map)))
}

fn part1((graph, mut value_map): (Graph<Node<'_>, ()>, HashMap<&str, u8>)) -> anyhow::Result<()> {
    for idx in toposort(&graph, None).map_err(|e| anyhow::anyhow!("{:?}", e))? {
        let node = &graph[idx];
        value_map.insert(node.name, node.op.result(&value_map));
    }

    let res = value_map
        .iter()
        .filter(|&(k, _)| k.starts_with('z'))
        .sorted_by_key(|&(k, _)| k)
        .enumerate()
        .fold(0u64, |acc, (i, (_, &v))| {
            acc + (2u64.pow(i as u32) * v as u64)
        });

    println!("{:?}", res);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day24.input");
    let (_, parsed) = parse(input)?;

    // println!("{:?}", parsed);

    part1(parsed)?;

    Ok(())
}
