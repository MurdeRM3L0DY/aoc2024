use std::collections::HashMap;

use nom::{
    Parser,
    character::complete::{space0, u32},
    multi::fold_many1,
    sequence::tuple,
};

fn parse(input: &str) -> nom::IResult<&str, HashMap<u64, u64>> {
    fold_many1(tuple((u32, space0)), HashMap::new, |mut acc, (n, _)| {
        acc.entry(n as u64).and_modify(|e| *e += 1).or_insert(1);
        acc
    })
    .parse(input)
}

fn split_num_in_half(n: u64) -> Option<(u32, u32)> {
    let count = n.checked_ilog10().unwrap_or(0) + 1;

    if count % 2 == 0 {
        let base = 10u64.pow(count / 2);
        Some(((n / base) as u32, (n % base) as u32))
    } else {
        None
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day11.input");
    let (_, mut parsed) = parse(input)?;
    println!("{:?}", parsed);

    for _ in 0..75 {
        let mut next = HashMap::new();

        for (n, count) in parsed {
            if n == 0 {
                next.entry(1).and_modify(|e| *e += count).or_insert(count);
            } else if let Some((l, r)) = split_num_in_half(n) {
                next.entry(l as u64)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                next.entry(r as u64)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            } else {
                next.entry(n * 2024)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }
        }

        parsed = next;
    }

    let count = parsed.values().sum::<u64>();
    println!("{:?}", count);
    Ok(())
}
