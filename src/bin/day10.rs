use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    E,
    N,
    W,
    S,
}

impl Direction {
    pub fn as_norm(&self) -> (i8, i8) {
        match self {
            Direction::E => (0, 1),
            Direction::N => (-1, 0),
            Direction::W => (0, -1),
            Direction::S => (1, 0),
        }
    }
}

#[derive(Debug, Default)]
struct InputData {
    start_pos: Vec<(u32, u32)>,
    pos_char_map: HashMap<(u32, u32), u8>,
    rows: u32,
    cols: u32,
}

fn parse(input: &str) -> InputData {
    input
        .lines()
        .enumerate()
        .fold(InputData::default(), |mut acc, (y, line)| {
            acc.rows += 1;

            if acc.cols == 0 {
                acc.cols = line.len() as u32;
            }

            for (x, c) in line.chars().enumerate() {
                let pos = (y as u32, x as u32);
                if c == '0' {
                    acc.start_pos.push(pos);
                }
                acc.pos_char_map.insert(pos, c as u8);
            }

            acc
        })
}

struct TrailData<T> {
    result: T,
    visited: HashMap<(u32, u32), u32>,
}

fn seq_in_dir_any(
    parsed: &InputData,
    pos: &(u32, u32),
    n: u8,
    data: &mut TrailData<&mut HashSet<(u32, u32)>>,
) {
    if data.visited.contains_key(pos) {
        return;
    }

    if parsed.pos_char_map.get(pos).unwrap() == &b'9' {
        data.result.insert((pos.0, pos.1));
        return;
    }

    for next_pos in [Direction::E, Direction::N, Direction::W, Direction::S]
        .into_iter()
        .filter_map(|dir| {
            let (dy, dx) = dir.as_norm();
            let next_pos = (pos.0 as i64 + dy as i64, pos.1 as i64 + dx as i64);

            let cond = !next_pos.0.is_negative()
                && !next_pos.1.is_negative()
                && (0..parsed.rows).contains(&(next_pos.0 as u32))
                && (0..parsed.cols).contains(&(next_pos.1 as u32))
                && parsed
                    .pos_char_map
                    .get(&(next_pos.0 as u32, next_pos.1 as u32))
                    .unwrap()
                    == &(n + 1);

            if cond {
                Some((next_pos.0 as u32, next_pos.1 as u32))
            } else {
                None
            }
        })
    {
        seq_in_dir_any(parsed, &next_pos, n + 1, data);
    }

    data.visited.insert((pos.0, pos.1), 1);
}

fn seq_in_dir_all(parsed: &InputData, pos: &(u32, u32), n: u8, data: &mut TrailData<()>) -> u32 {
    if let Some(res) = data.visited.get(pos) {
        return *res;
    }

    if parsed.pos_char_map.get(pos).unwrap() == &b'9' {
        return 1;
    }

    let result = [Direction::E, Direction::N, Direction::W, Direction::S]
        .into_iter()
        .filter_map(|dir| {
            let (dy, dx) = dir.as_norm();
            let next_pos = (pos.0 as i64 + dy as i64, pos.1 as i64 + dx as i64);

            let cond = !next_pos.0.is_negative()
                && !next_pos.1.is_negative()
                && (0..parsed.rows).contains(&(next_pos.0 as u32))
                && (0..parsed.cols).contains(&(next_pos.1 as u32))
                && parsed
                    .pos_char_map
                    .get(&(next_pos.0 as u32, next_pos.1 as u32))
                    .unwrap()
                    == &(n + 1);

            if cond {
                Some((next_pos.0 as u32, next_pos.1 as u32))
            } else {
                None
            }
        })
        .map(|next_pos| seq_in_dir_all(parsed, &next_pos, n + 1, data))
        .sum::<u32>();

    data.visited.entry((pos.0, pos.1)).or_insert(result);

    result
}

fn part1(parsed: &InputData) {
    let result = parsed.start_pos.iter().fold(
        HashMap::<&(u32, u32), HashSet<(u32, u32)>>::new(),
        |mut acc, pos| {
            seq_in_dir_any(parsed, pos, b'0', &mut TrailData {
                result: acc.entry(pos).or_default(),
                visited: HashMap::new(),
            });
            acc
        },
    );

    let count = result.values().map(|v| v.len() as u32).sum::<u32>();

    println!("part1: {}", count);
}

fn part2(parsed: &InputData) {
    let result = parsed.start_pos.iter().fold(0, |acc, pos| {
        acc + seq_in_dir_all(parsed, pos, b'0', &mut TrailData {
            result: (),
            visited: HashMap::new(),
        })
    });
    println!("part2: {}", result);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day10.input");
    let parsed = parse(input);
    // println!("parsed={:?}", parsed);
    part1(&parsed);
    part2(&parsed);
    Ok(())
}
