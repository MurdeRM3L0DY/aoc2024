use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;
use nom::{
    character::complete::{line_ending, u32},
    multi::many1,
    sequence::terminated,
};

fn parse(input: &str) -> nom::IResult<&str, Vec<u32>> {
    many1(terminated(u32, line_ending))(input)
}

#[allow(clippy::let_and_return)]
fn next_secret(n: u32) -> u32 {
    // 16777216 = 2^24
    //
    // n mod d == n & (d - 1); for d = 2^e
    //
    // let m := (16777216 - 1) = 0b01111111_11111111_11111111;
    //
    // ex: let n := 1;
    //      (1 ^ 1 << 6)
    //  0b00000000_00000000_01000001 & 0b01111111_11111111_11111111 =
    //      0b00000000_00000000_01000001
    //      (65 ^ 65 >> 5)
    //  0b00000000_00000000_01000011 & 0b01111111_11111111_11111111 =
    //      0b00000000_00000000_01000011
    //      (67 ^ 67 << 11)
    //  0b00000010_00011000_01000011 & 0b01111111_11111111_11111111 =
    //      0b00000010_00011000_01000011

    let mask = 16777216 - 1; // 0b01111111_11111111_11111111;
    let n = (n ^ n << 6) & mask;
    let n = (n ^ n >> 5) & mask;
    let n = (n ^ n << 11) & mask;
    n
}

fn part2(parsed: &[u32]) {
    let mut prices = HashMap::<[i8; 4], (HashSet<usize>, u32)>::new();
    let count = 2000;
    for (i, succ) in parsed
        .iter()
        .map(|&n| {
            iter::successors(Some(n), |&n| Some(next_secret(n)))
                .tuple_windows::<(_, _, _, _, _)>()
                .take((count + 1) - 4)
        })
        .enumerate()
    {
        for (a, b, c, d, e) in succ {
            let k = [
                (b % 10) as i8 - (a % 10) as i8,
                (c % 10) as i8 - (b % 10) as i8,
                (d % 10) as i8 - (c % 10) as i8,
                (e % 10) as i8 - (d % 10) as i8,
            ];
            let v = e % 10;

            prices
                .entry(k)
                .and_modify(|e| {
                    if !e.0.contains(&i) {
                        e.0.insert(i);
                        e.1 += v;
                    }
                })
                .or_insert((HashSet::from_iter([i]), v));
        }
    }

    if let Some((key, value)) = prices.iter().max_by(|a, b| a.1.1.cmp(&b.1.1)) {
        println!(
            "The key with the maximum value is: {:?} with value {:?}",
            key, value.1
        );
    } else {
        println!("The map is empty.");
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day22.input");
    let (_, parsed) = parse(input)?;
    // println!("{:?}", parsed);
    part2(&parsed);
    Ok(())
}
