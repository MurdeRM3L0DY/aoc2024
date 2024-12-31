use core::hash;
use std::collections::{HashMap, HashSet};
use std::iter;

use glam::{I8Vec2, I16Vec2, U8Vec2};
use indexmap::IndexMap;
use nom::{
    bytes::complete::is_not, character::complete::line_ending, combinator::map, multi::fold_many1,
    sequence::tuple,
};

#[derive(Debug)]
struct Parsed {
    walls: HashSet<U8Vec2>,
    start_pos: U8Vec2,
    goal_pos: U8Vec2,
}

fn parse(input: &str) -> nom::IResult<&str, Parsed> {
    map(
        fold_many1(
            tuple((is_not("\n"), line_ending)),
            || {
                (
                    Parsed {
                        walls: HashSet::new(),
                        start_pos: U8Vec2::MAX,
                        goal_pos: U8Vec2::MAX,
                    },
                    0,
                )
            },
            |(mut parsed, mut y), (line, _): (&str, &str)| {
                for (x, c) in line.chars().enumerate() {
                    let pos = U8Vec2::new(x as u8, y as u8);

                    if c == '#' {
                        parsed.walls.insert(pos);
                    } else if c == 'S' {
                        parsed.start_pos = pos;
                    } else if c == 'E' {
                        parsed.goal_pos = pos
                    }
                }

                y += 1;

                (parsed, y)
            },
        ),
        |r| r.0,
    )(input)
}

#[derive(Debug, Clone, Copy)]
struct Node {
    pos: U8Vec2,
    // dir: I8Vec2,
    path_cost: u32,
}

impl hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.path_cost.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Node {}

fn build_path(parents: &IndexMap<Node, usize>, mut i: usize) -> Vec<&Node> {
    let mut v = iter::from_fn(|| {
        parents.get_index(i).map(|(node, index)| {
            i = *index;
            node
        })
    })
    .collect::<Vec<_>>();

    v.reverse();

    v
}

fn solve(parsed: &Parsed) {
    let mut i = 0;
    let mut parents = IndexMap::new();
    parents.insert(
        Node {
            pos: parsed.start_pos,
            path_cost: 0,
        },
        usize::MAX,
    );

    let mut seen = HashSet::new();

    let path = loop {
        if let Some((Node { pos, path_cost }, _)) = parents.get_index(i) {
            if pos == &parsed.goal_pos {
                break build_path(&parents, i);
            }

            if seen.contains(pos) {
                i += 1;
                continue;
            }

            seen.insert(*pos);

            let pos = *pos;
            let path_cost = *path_cost;

            let next_path_cost = path_cost + 1;
            for dir in [I8Vec2::X, I8Vec2::NEG_Y, I8Vec2::NEG_X, I8Vec2::Y] {
                let next_pos = pos.as_i16vec2() + dir.as_i16vec2();
                if next_pos.is_negative_bitmask() > 0 {
                    continue;
                }
                let next_pos = next_pos.as_u8vec2();

                if !parsed.walls.contains(&next_pos) {
                    parents
                        .entry(Node {
                            pos: next_pos,
                            path_cost: next_path_cost,
                        })
                        .or_insert(i);
                }
            }

            i += 1;
        }
    };

    let path_costs = path
        .iter()
        .map(|n| (n.pos, n.path_cost))
        .collect::<HashMap<_, _>>();
    let total_cost = path_costs[&parsed.goal_pos];
    println!("total_cost={}", total_cost);

    let radius = 20i32;
    let mut count = 0;
    let saved = 100;
    for node in &path {
        for y in -radius..=radius {
            let max_x = radius - y.abs();
            for x in -max_x..=max_x {
                let npos = node.pos.as_i16vec2() + I16Vec2::new(x as i16, y as i16);
                if npos.is_negative_bitmask() > 0 {
                    continue;
                }
                let npos = npos.as_u8vec2();

                if let Some(&cheat_pos_path_cost) = path_costs.get(&npos) {
                    let remaining_path_cost = total_cost - cheat_pos_path_cost;
                    // let path_cost_from_start_pos = total_cost - node.path_cost;
                    let new_path_cost =
                        remaining_path_cost + node.path_cost + x.unsigned_abs() + y.unsigned_abs();

                    // println!(
                    //     "from={:?}, to={:?}, path_cost_from_cheat_pos={}, new_path_cost={}",
                    //     node.pos,
                    //     npos,
                    //     path_cost_from_cheat_pos,
                    //     node.path_cost + path_cost_from_cheat_pos + radius as u32,
                    // );

                    if new_path_cost <= total_cost - saved {
                        // println!("from={:?}, to={:?}", node.pos, npos);
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{} cheats saved at least {} picoseconds", count, saved);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day20.input");
    let (_, parsed) = parse(input)?;
    // println!("{:?}", parsed);

    solve(&parsed);

    Ok(())
}
