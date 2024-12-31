use std::collections::{BinaryHeap, HashMap, HashSet};

use glam::{I8Vec2, U8Vec2};
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u8},
    multi::{fold_many1, many1},
    sequence::{separated_pair, terminated},
};

fn parse(input: &str) -> nom::IResult<&str, Vec<U8Vec2>> {
    many1(terminated(
        separated_pair(u8, tag(","), u8).map(|(x, y)| U8Vec2::new(x, y)),
        line_ending,
    ))(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: U8Vec2,
    dir: I8Vec2,
    path_cost: u32,
    estimated_cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            std::cmp::Ordering::Equal => self.path_cost.cmp(&other.path_cost),
            o => o,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn path_cost_for_n_bytes(parsed: &[U8Vec2], n: usize) -> Option<u32> {
    let bytes = parsed.iter().take(n).collect::<HashSet<_>>();
    let heu = |a: U8Vec2, b: U8Vec2| -> u32 {
        let (ax, ay) = (a.x as i16, a.y as i16);
        let (bx, by) = (b.x as i16, b.y as i16);

        (ax.abs_diff(bx) + ay.abs_diff(by)) as u32
    };

    let mut costs = HashMap::new();
    let mut min_cost = None;

    let start_pos = U8Vec2::new(0, 0);
    let goal_pos = U8Vec2::new(70, 70);

    let mut queue = BinaryHeap::new();
    queue.push(Node {
        pos: start_pos,
        dir: I8Vec2::Y,
        path_cost: 0,
        estimated_cost: heu(start_pos, goal_pos),
    });

    let mut predecessors = HashMap::new();

    while let Some(Node {
        pos,
        dir,
        path_cost,
        estimated_cost,
    }) = queue.pop()
    {
        if matches!(min_cost, Some(min_cost) if estimated_cost > min_cost) {
            continue;
        }

        if pos == goal_pos {
            min_cost = Some(path_cost);
        }
        for next_dir in [dir, dir.perp(), -dir.perp()] {
            // println!("{:?}, {:?}, {:?}, {:?}", dir, dir.perp(), -dir.perp(), pos);
            let next_pos = pos.as_i16vec2() + next_dir.as_i16vec2();
            if next_pos.is_negative_bitmask() > 0 {
                continue;
            }

            let next_pos = next_pos.as_u8vec2();

            if next_pos.x > goal_pos.x || next_pos.y > goal_pos.y {
                continue;
            }

            if bytes.contains(&next_pos) {
                continue;
            }

            predecessors
                .entry(next_pos)
                .and_modify(|e: &mut HashSet<U8Vec2>| {
                    e.insert(pos);
                })
                .or_default()
                .insert(pos);

            let new_path_cost = path_cost + 1;
            let h = heu(next_pos, goal_pos);

            if let Some(c) = costs.get_mut(&next_pos) {
                if *c > new_path_cost {
                    *c = new_path_cost;
                    queue.push(Node {
                        pos: next_pos,
                        dir: next_dir,
                        path_cost: new_path_cost,
                        estimated_cost: new_path_cost + h,
                    });
                }
            } else {
                costs.insert(next_pos, new_path_cost);
                queue.push(Node {
                    pos: next_pos,
                    dir: next_dir,
                    path_cost: new_path_cost,
                    estimated_cost: new_path_cost + h,
                });
            }
        }
    }

    min_cost
}

fn part1(parsed: &[U8Vec2]) {
    let min_cost = path_cost_for_n_bytes(parsed, 1024);

    println!("part1: {:?}", min_cost);
}

fn part2(parsed: &[U8Vec2]) {
    let mut l = 1025;
    let mut h = parsed.len() - 1;

    while l < h {
        let m = (l + h) / 2;
        if path_cost_for_n_bytes(parsed, m + 1).is_some() {
            l = m + 1;
        } else {
            h = m;
        }
    }

    println!("part2: {},{} (i={})", parsed[l].x, parsed[l].y, l);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day18.input");
    let (_, parsed) = parse(input)?;
    // println!("{:?}", parsed);
    part1(&parsed);
    part2(&parsed);
    Ok(())
}
