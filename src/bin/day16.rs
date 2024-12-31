use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use glam::{I8Vec2, Mat2, U8Vec2, Vec2};
use nom::{
    bytes::complete::is_not, character::complete::line_ending, combinator::map, multi::fold_many1,
    sequence::tuple,
};

#[derive(Debug)]
struct Parsed {
    pos_set: HashMap<U8Vec2, char>,
    start_pos: U8Vec2,
    goal_pos: U8Vec2,
}

fn parse(input: &str) -> nom::IResult<&str, Parsed> {
    map(
        fold_many1(
            tuple((is_not("\n"), line_ending)),
            || (HashMap::new(), 0, U8Vec2::MAX, U8Vec2::MAX),
            |(mut set, mut y, mut start_pos, mut goal_pos), (line, _): (&str, &str)| {
                for (x, c) in line.chars().enumerate() {
                    let pos = U8Vec2::new(x as u8, y as u8);
                    set.insert(pos, c);

                    if c == 'S' {
                        start_pos = pos;
                    } else if c == 'E' {
                        goal_pos = pos;
                    }
                }
                y += 1;
                (set, y, start_pos, goal_pos)
            },
        ),
        |(pos_set, _, start_pos, goal_pos)| Parsed {
            pos_set,
            start_pos,
            goal_pos,
        },
    )(input)
}

// --------> (x)
// |
// |
// |
// |
// |
// v
// (y)

const ROT_90_CW: Mat2 = Mat2::from_cols(Vec2::new(0.0, 1.0), Vec2::new(-1.0, 0.0));
const ROT_90_CCW: Mat2 = Mat2::from_cols(Vec2::new(0.0, -1.0), Vec2::new(1.0, 0.0));

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

fn part1(parsed: Parsed) {
    let heu = |a: U8Vec2, b: U8Vec2| -> u32 {
        let (ax, ay) = (a.x as i16, a.y as i16);
        let (bx, by) = (b.x as i16, b.y as i16);

        (ax.abs_diff(bx) + ay.abs_diff(by)) as u32
    };

    let mut costs = HashMap::new();
    let mut min_cost = None;

    let mut predecessors = HashMap::new();
    // predecessors.insert(parsed.start_pos, HashSet::new());

    let mut q = BinaryHeap::new();
    q.push(Node {
        pos: parsed.start_pos,
        dir: I8Vec2::X,
        path_cost: 0,
        estimated_cost: heu(parsed.start_pos, parsed.goal_pos),
    });

    while let Some(Node {
        pos,
        dir,
        path_cost,
        estimated_cost,
    }) = q.pop()
    {
        if matches!(min_cost, Some(min_cost) if estimated_cost > min_cost) {
            continue;
        }

        if pos == parsed.goal_pos {
            min_cost = Some(path_cost);
        }

        // NOTE: could've also used `dir.perp()` and `-dir.perp()`
        for (next_dir, next_path_cost) in [
            (dir.as_vec2(), path_cost + 1),
            (ROT_90_CCW.mul_vec2(dir.as_vec2()), (path_cost + 1) + 1000),
            (ROT_90_CW.mul_vec2(dir.as_vec2()), (path_cost + 1) + 1000),
        ] {
            let next_pos = pos.as_i16vec2() + next_dir.as_i16vec2();
            if next_pos.is_negative_bitmask() == 0
                && parsed
                    .pos_set
                    .get(&next_pos.as_u8vec2())
                    .is_some_and(|c| *c != '#')
            {
                let next_pos = next_pos.as_u8vec2();
                let next_dir = next_dir.as_i8vec2();

                predecessors
                    .entry(next_pos)
                    .and_modify(|e: &mut HashSet<U8Vec2>| {
                        e.insert(pos);
                    })
                    .or_default();

                let h = heu(next_pos, parsed.goal_pos);

                if let Some(c) = costs.get_mut(&next_pos) {
                    if *c > next_path_cost {
                        *c = next_path_cost;
                        q.push(Node {
                            pos: next_pos,
                            dir: next_dir,
                            path_cost: next_path_cost,
                            estimated_cost: next_path_cost + h,
                        });
                    }
                } else {
                    costs.insert(pos, next_path_cost);
                    q.push(Node {
                        pos: next_pos,
                        dir: next_dir,
                        path_cost: next_path_cost,
                        estimated_cost: next_path_cost + h,
                    });
                }
            }
        }
    }

    // let mut ris = HashSet::new();
    // count(&parsed.goal_pos, &predecessors, &mut ris);
    //
    // println!("{:?}", ris.len());
    println!("part1: {:?}", min_cost);
}

fn count<'a>(
    pos: &'a U8Vec2,
    predec: &'a HashMap<U8Vec2, HashSet<U8Vec2>>,
    ris: &mut HashSet<&'a U8Vec2>,
) {
    if ris.contains(pos) {
        return;
    }
    ris.insert(pos);

    if let Some(v) = predec.get(pos) {
        println!("{:?}", v);
        for p in v {
            count(p, predec, ris);
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day16.sample");
    let (_, parsed) = parse(input)?;
    // println!("{:?}", parsed);
    part1(parsed);

    Ok(())
}
