use std::iter;
use std::ops::Range;

use nom::{
    Parser, bytes::complete::is_not, character::complete::line_ending, combinator::opt,
    multi::fold_many1, sequence::tuple,
};

const ROWS: usize = 7;
const COLS: usize = 5;
const SIZE: usize = ROWS * COLS;
fn parse_grid(input: &str) -> nom::IResult<&str, [u8; SIZE]> {
    fold_many1(
        tuple((is_not("\n"), line_ending)),
        || ([u8::MAX; SIZE], 0),
        |(mut acc, mut y), (line, _): (&str, &str)| {
            let idx = y * COLS..(y + 1) * COLS;
            acc[idx].copy_from_slice(line.as_bytes());

            y += 1;

            (acc, y)
        },
    )
    .map(|(grid, _)| grid)
    .parse(input)
}

#[derive(Debug)]
struct Parsed {
    locks: Vec<[u8; COLS]>,
    keys: Vec<[u8; COLS]>,
}

const TOP_ROW: Range<usize> = 0..COLS;
const BOT_ROW: Range<usize> = SIZE - COLS..SIZE;
fn parse(input: &str) -> nom::IResult<&str, Parsed> {
    fold_many1(
        tuple((parse_grid, opt(line_ending))),
        || Parsed {
            locks: vec![],
            keys: vec![],
        },
        |mut parsed, (grid, _)| {
            let s = if &grid[TOP_ROW] == b"#####" {
                &mut parsed.locks
            } else if &grid[BOT_ROW] == b"#####" {
                &mut parsed.keys
            } else {
                unreachable!("skill issue");
            };

            let mut v = [0; COLS];
            for y in 0..ROWS {
                for x in 0..COLS {
                    if grid[y * COLS + x] == b'#' {
                        v[x] += 1;
                    }
                }
            }
            for i in v.iter_mut().take(COLS) {
                *i -= 1;
            }

            s.push(v);

            parsed
        },
    )(input)
}

fn solve(parsed: &Parsed) {
    let result = itertools::iproduct!(&parsed.locks, &parsed.keys)
        .filter(|&(lock, key)| {
            iter::zip(lock, key)
                .map(|(l, k)| l + k)
                .all(|n| n <= ROWS as u8 - 2)
        })
        .count();
    println!("result={}", result);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day25.input");
    let (_, parsed) = parse(input)?;

    // println!("{:?}", parsed);

    solve(&parsed);
    Ok(())
}
