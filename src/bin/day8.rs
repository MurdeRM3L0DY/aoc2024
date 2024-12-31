use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Default)]
struct InputData {
    char_pos_map: HashMap<u8, Vec<(u32, u32)>>,
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
                match c {
                    ('A'..='Z') | ('a'..='z') | ('0'..='9') => {
                        acc.pos_char_map.insert(pos, c as u8);
                        acc.char_pos_map.entry(c as u8).or_default().push(pos);
                    }
                    _ => {}
                }
            }

            acc
        })
}

fn part1(parsed: &InputData) {
    let mut result = HashSet::new();

    for (_, points) in parsed.char_pos_map.iter() {
        for p in points.iter().combinations(2) {
            let a = p[0];
            let b = p[1];

            let dy = b.0 as i64 - a.0 as i64;
            let dx = b.1 as i64 - a.1 as i64;

            // println!("p={:?}, c={}, dy={:?}, dx={:?}", p, c, dy, dx);

            let p1 = (a.0 as i64 - dy, a.1 as i64 - dx);
            if !p1.0.is_negative()
                && !p1.1.is_negative()
                && (0..parsed.rows).contains(&(p1.0 as u32))
                && (0..parsed.cols).contains(&(p1.1 as u32))
            {
                // println!("p1={:?}", p1);
                result.insert(p1);
            }

            let p2 = (b.0 as i64 + dy, b.1 as i64 + dx);
            if !p2.0.is_negative()
                && !p2.1.is_negative()
                && (0..parsed.rows).contains(&(p2.0 as u32))
                && (0..parsed.cols).contains(&(p2.1 as u32))
            {
                // println!("p2={:?}", p2);
                result.insert(p2);
            }
        }

        // println!("------------------------------------------------")
    }

    println!("part1: {}", result.len());
}

fn part2(parsed: &InputData) {
    let mut result = HashSet::new();

    for (c, points) in parsed.char_pos_map.iter() {
        for p in points.iter().combinations(2) {
            let a = p[0];
            let b = p[1];

            let dy = b.0 as i64 - a.0 as i64;
            let dx = b.1 as i64 - a.1 as i64;

            // println!("p={:?}, c={}, dy={:?}, dx={:?}", p, c, dy, dx);

            let mut p1 = (a.0 as i64 - dy, a.1 as i64 - dx);
            loop {
                if !p1.0.is_negative()
                    && !p1.1.is_negative()
                    && (0..parsed.rows).contains(&(p1.0 as u32))
                    && (0..parsed.cols).contains(&(p1.1 as u32))
                {
                    // println!("p1={:?}", p1);
                    result.insert(p1);
                    p1 = (p1.0 - dy, p1.1 - dx);
                } else {
                    break;
                }
            }

            let mut p2 = (b.0 as i64 + dy, b.1 as i64 + dx);
            loop {
                if !p2.0.is_negative()
                    && !p2.1.is_negative()
                    && (0..parsed.rows).contains(&(p2.0 as u32))
                    && (0..parsed.cols).contains(&(p2.1 as u32))
                {
                    // println!("p2={:?}", p2);
                    result.insert(p2);
                    p2 = (p2.0 + dy, p2.1 + dx);
                } else {
                    break;
                }
            }

            result.insert((a.0 as i64, a.1 as i64));
            result.insert((b.0 as i64, b.1 as i64));
        }

        // println!("------------------------------------------------")
    }

    println!("part1: {}", result.len());
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day8.input");
    let data = parse(input);

    println!("{:?}", data);

    part1(&data);
    part2(&data);
    Ok(())
}
