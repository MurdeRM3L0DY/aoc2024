use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Default)]
struct InputData {
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
                acc.pos_char_map.insert(pos, c as u8);
            }

            acc
        })
}

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

fn get_connected_iter(
    pos: &(u32, u32),
    parsed: &InputData,
    regions: &mut HashMap<(u32, u32), u32>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(*pos);

    while let Some(pos) = queue.pop_front() {
        if regions.contains_key(&pos) {
            continue;
        }

        regions.insert(pos, 0);

        for next_pos in [Direction::E, Direction::N, Direction::W, Direction::S]
            .into_iter()
            .filter_map(|dir| {
                let (dy, dx) = dir.as_norm();
                let next_pos = (pos.0 as i64 + dy as i64, pos.1 as i64 + dx as i64);

                let cond = !next_pos.0.is_negative()
                    && !next_pos.1.is_negative()
                    && (0..parsed.rows).contains(&(next_pos.0 as u32))
                    && (0..parsed.cols).contains(&(next_pos.1 as u32))
                    && parsed.pos_char_map.get(&pos).unwrap()
                        == parsed
                            .pos_char_map
                            .get(&(next_pos.0 as u32, next_pos.1 as u32))
                            .unwrap();

                if cond {
                    Some((next_pos.0 as u32, next_pos.1 as u32))
                } else {
                    None
                }
            })
        {
            regions.entry(pos).and_modify(|e| {
                *e += 1;
            });

            queue.push_back(next_pos);
        }
    }
}

fn part1(parsed: &InputData) {
    let mut curr_pos = parsed.pos_char_map.keys().collect::<Vec<_>>();
    let mut price = 0;
    let mut connected = HashMap::new();
    let mut visited = HashSet::new();
    while let Some(pos) = curr_pos.pop() {
        if visited.contains(pos) {
            continue;
        }

        get_connected_iter(pos, parsed, &mut connected);

        let area = connected.len() as u32;
        let perimeter = connected.values().map(|neigh| 4 - *neigh).sum::<u32>();
        // println!("{} * {}", area, perimeter);
        price += area * perimeter;

        visited.extend(connected.drain().map(|(k, _)| k));
    }

    println!("part1: {:?}", price);
}

fn part2(parsed: &InputData) {
    let mut curr_pos = parsed.pos_char_map.keys().collect::<Vec<_>>();
    let mut price = 0;
    let mut visited = HashSet::new();
    let mut connected = HashMap::new();
    while let Some(pos) = curr_pos.pop() {
        if visited.contains(pos) {
            continue;
        }

        get_connected_iter(pos, parsed, &mut connected);

        let area = connected.len() as u32;
        let mut sides = 0;
        let c = parsed.pos_char_map.get(pos);
        for (pos, _) in connected.drain() {
            visited.insert(pos);

            let pos = (pos.0 as i64, pos.1 as i64);
            // directions are intentionally 90 degrees offset of each other
            for (d1, d2) in [Direction::E, Direction::N, Direction::W, Direction::S]
                .into_iter()
                .circular_tuple_windows()
            {
                let (d1y, d1x) = d1.as_norm();
                let (d2y, d2x) = d2.as_norm();

                let d1y = d1y as i64;
                let d1x = d1x as i64;
                let d2y = d2y as i64;
                let d2x = d2x as i64;

                let n1 = (pos.0 + d1y, pos.1 + d1x);
                let n2 = (pos.0 + d2y, pos.1 + d2x);
                let nd = (pos.0 + d1y + d2y, pos.1 + d1x + d2x);

                let n1c = (!n1.0.is_negative()
                    && !n1.1.is_negative()
                    && (0..parsed.rows).contains(&(n1.0 as u32))
                    && (0..parsed.cols).contains(&(n1.1 as u32)))
                .then(|| {
                    // println!("{:?}, rows={}, cols={}", n1, parsed.rows, parsed.cols);

                    parsed
                        .pos_char_map
                        .get(&(n1.0 as u32, n1.1 as u32))
                        .expect("there should be an entry in the map")
                });

                let n2c = (!n2.0.is_negative()
                    && !n2.1.is_negative()
                    && (0..parsed.rows).contains(&(n2.0 as u32))
                    && (0..parsed.cols).contains(&(n2.1 as u32)))
                .then(|| {
                    // println!("{:?}, rows={}, cols={}", n2, parsed.rows, parsed.cols);

                    parsed
                        .pos_char_map
                        .get(&(n2.0 as u32, n2.1 as u32))
                        .expect("there should be an entry in the map")
                });

                let outer_edge = (c != n1c) && (c != n2c);
                let inner_edge = c == n1c
                    && c == n2c
                    && !nd.0.is_negative()
                    && !nd.1.is_negative()
                    && (0..parsed.rows).contains(&(nd.0 as u32))
                    && (0..parsed.cols).contains(&(nd.1 as u32))
                    && c != parsed.pos_char_map.get(&(nd.0 as u32, nd.1 as u32));

                if outer_edge {
                    // println!(
                    //     "[outer] pos={:?} n1={:?}, n2={:?}, nd={:?}, dir={:?}",
                    //     pos,
                    //     n1,
                    //     n2,
                    //     nd,
                    //     (d1, d2),
                    // );
                    sides += 1;
                }
                if inner_edge {
                    // println!(
                    //     "[inner] pos={:?}, n1={:?}, n2={:?}, nd={:?}, dir={:?}",
                    //     pos,
                    //     n1,
                    //     n2,
                    //     nd,
                    //     (d1, d2)
                    // );
                    sides += 1;
                }
            }
        }

        // println!("{} * {} -- {:?}", area, sides, c);
        price += area * sides;
    }

    println!("part2: {:?}", price);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day12.input");
    let parsed = parse(input);
    part1(&parsed);
    part2(&parsed);
    Ok(())
}
