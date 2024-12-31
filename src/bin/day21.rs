use std::collections::VecDeque;
use std::iter;
use std::{collections::HashMap, hash::Hash};

use glam::{I8Vec2, Mat2, U8Vec2, Vec2};
use indexmap::IndexMap;
use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, line_ending},
    multi::many1,
    sequence::terminated,
};

fn parse(input: &str) -> nom::IResult<&str, Vec<&str>> {
    many1(terminated(alphanumeric1, line_ending))(input)
}

#[derive(Debug)]
struct Node {
    pos: U8Vec2,
    seq: String,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

#[derive(Debug)]
struct CGrid {
    data: IndexMap<char, U8Vec2>,
    width: u8,
}

impl CGrid {
    fn from_iter(iter: impl IntoIterator<Item = (char, U8Vec2)>, width: u8) -> Self {
        Self {
            data: IndexMap::from_iter(iter),
            width,
        }
    }

    fn char(&self, pos: U8Vec2) -> Option<&char> {
        let idx = pos.y * self.width + pos.x;
        self.data.get_index(idx as usize).map(|(c, _)| c)
    }

    fn pos(&self, c: &char) -> Option<&U8Vec2> {
        self.data.get(c)
    }
}

fn build_path(q: &IndexMap<Node, usize>, pad: &CGrid, mut i: usize) -> Vec<char> {
    let mut v = iter::from_fn(|| {
        q.get_index(i).map(|(node, idx)| {
            i = *idx;
            *pad.char(node.pos).expect("Invalid key")
        })
    })
    .collect::<Vec<_>>();

    v.reverse();

    v
}

const FLIP_Y: Mat2 = Mat2::from_cols(Vec2::new(1.0, 0.0), Vec2::new(0.0, -1.0));

fn compute_map(pad: &CGrid) -> HashMap<(char, char), Vec<String>> {
    let mut r = HashMap::new();

    for (s, e) in itertools::iproduct!(pad.data.keys(), pad.data.keys()) {
        let spos = *pad.pos(s).expect("Invalid key");
        let epos = *pad.pos(e).expect("Invalid key");

        let diff = epos.as_i16vec2() - spos.as_i16vec2();
        let mask = diff.is_negative_bitmask();

        let dirs = if diff.x == 0 && diff.y != 0 {
            [
                None,
                (mask == 2)
                    .then_some((I8Vec2::NEG_Y, '^'))
                    .or(Some((I8Vec2::Y, 'v'))),
            ]
        } else if diff.y == 0 && diff.x != 0 {
            [
                (mask == 1)
                    .then_some((I8Vec2::NEG_X, '<'))
                    .or(Some((I8Vec2::X, '>'))),
                None,
            ]
        } else {
            match mask {
                0 => [Some((I8Vec2::X, '>')), Some((I8Vec2::Y, 'v'))],
                1 => [Some((I8Vec2::NEG_X, '<')), Some((I8Vec2::Y, 'v'))],
                2 => [Some((I8Vec2::X, '>')), Some((I8Vec2::NEG_Y, '^'))],
                3 => [Some((I8Vec2::NEG_X, '<')), Some((I8Vec2::NEG_Y, '^'))],
                _ => unreachable!("skill issue"),
            }
        };

        let mut q = VecDeque::new();
        q.push_back(Node {
            pos: spos,
            seq: String::new(),
        });

        while let Some(mut node) = q.pop_front() {
            if node.pos == epos {
                node.seq.push('A');
                r.entry((*s, *e)).or_insert(Vec::new()).push(node.seq);
                continue;
            }

            for (dir, c) in dirs.into_iter().flatten() {
                let next_pos = node.pos.as_i16vec2() + dir.as_i16vec2();
                if next_pos.is_negative_bitmask() > 0 {
                    continue;
                }
                let next_pos = next_pos.as_u8vec2();

                if pad.char(next_pos).is_some_and(|nc| nc != &'-') {
                    let mut seq = node.seq.clone();
                    seq.push(c);
                    q.push_back(Node { pos: next_pos, seq });
                }
            }
        }
    }

    r
}

fn minlengthd<'a>(
    d: u32,
    pseq: &'a String,
    dmap: &'a HashMap<(char, char), Vec<String>>,
    cache: &mut HashMap<(&'a String, u32), u64>,
) -> u64 {
    if let Some(l) = cache.get(&(pseq, d)) {
        return *l;
    }

    let inseq = format!("A{}", pseq);
    if d == 1 {
        inseq
            .chars()
            .tuple_windows()
            .map(|(s, e)| {
                // all seqs at depth 1 have the same length
                dmap[&(s, e)][0].len() as u64
            })
            .sum()
    } else {
        let mut minlength = 0;

        for (s, e) in inseq.chars().tuple_windows() {
            minlength += dmap[&(s, e)]
                .iter()
                .map(|cseq| minlengthd(d - 1, cseq, dmap, cache))
                .min()
                .expect("Unable to calculate minimum");
        }

        cache.insert((pseq, d), minlength);

        minlength
    }
}

fn solve(
    parsed: &[&str],
    maps: &(
        HashMap<(char, char), Vec<String>>,
        HashMap<(char, char), Vec<String>>,
    ),
) -> anyhow::Result<()> {
    let (nmap, dmap) = maps;

    let pseqs = parsed
        .iter()
        .map(|line| {
            format!("A{}", line)
                .chars()
                .tuple_windows()
                .map(|(s, e)| &nmap[&(s, e)])
                .multi_cartesian_product()
                .map(|pseqs| pseqs.into_iter().join(""))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut minlengths = vec![];
    let mut cache = HashMap::new();
    for pseq in &pseqs {
        let mut minlength = u64::MAX;
        for seq in pseq {
            minlength = minlength.min(minlengthd(25, seq, dmap, &mut cache));
        }

        minlengths.push(minlength);
    }

    let mut result = 0;
    for (minlength, line) in iter::zip(minlengths, parsed) {
        let n = line[..line.len() - 1].parse::<u64>()?;
        result += minlength * n;
    }

    println!("{}", result);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day21.input");
    let (_, parsed) = parse(input)?;

    println!("{:?}", parsed);

    let npad = CGrid::from_iter(
        [
            ('7', U8Vec2::new(0, 0)),
            ('8', U8Vec2::new(1, 0)),
            ('9', U8Vec2::new(2, 0)),
            ('4', U8Vec2::new(0, 1)),
            ('5', U8Vec2::new(1, 1)),
            ('6', U8Vec2::new(2, 1)),
            ('1', U8Vec2::new(0, 2)),
            ('2', U8Vec2::new(1, 2)),
            ('3', U8Vec2::new(2, 2)),
            ('-', U8Vec2::new(0, 3)),
            ('0', U8Vec2::new(1, 3)),
            ('A', U8Vec2::new(2, 3)),
        ],
        3,
    );
    let npad_map = compute_map(&npad);

    let dpad = CGrid::from_iter(
        [
            ('-', U8Vec2::new(0, 0)),
            ('^', U8Vec2::new(1, 0)),
            ('A', U8Vec2::new(2, 0)),
            ('<', U8Vec2::new(0, 1)),
            ('v', U8Vec2::new(1, 1)),
            ('>', U8Vec2::new(2, 1)),
        ],
        3,
    );
    let dpad_map = compute_map(&dpad);

    solve(&parsed, &(npad_map, dpad_map))?;

    Ok(())
}
