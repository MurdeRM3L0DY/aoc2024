use std::collections::{HashMap, HashSet};

use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{terminated, tuple},
};

fn parse(input: &str) -> nom::IResult<&str, (HashSet<&str>, Vec<&str>)> {
    let (input, patterns) = terminated(
        fold_many1(
            tuple((alpha1, opt(tag(", ")))).map(|(t, _)| t),
            HashSet::new,
            |mut acc, t| {
                acc.insert(t);
                acc
            },
        ),
        line_ending,
    )(input)?;

    let (input, _) = line_ending(input)?;

    let (input, towels) = separated_list1(line_ending, alpha1)(input)?;

    Ok((input, (patterns, towels)))
}

fn part1(parsed: &(HashSet<&str>, Vec<&str>)) {
    let (patterns, towels) = parsed;

    let mut cache = HashMap::new();

    let result = towels
        .iter()
        .filter(|t| design_is_valid(t, patterns, &mut cache))
        .count();

    println!("part1: {}", result);
}

fn design_is_valid<'a>(
    t: &'a str,
    patterns: &HashSet<&'a str>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if patterns.contains(t) {
        return true;
    }

    if let Some(b) = cache.get(t) {
        return *b;
    }

    for i in 1..t.len() {
        let (l, r) = t.split_at(i);

        let l_is_valid = design_is_valid(l, patterns, cache);
        cache.insert(l, l_is_valid);
        if !l_is_valid {
            continue;
        }

        let r_is_valid = design_is_valid(r, patterns, cache);
        cache.insert(r, r_is_valid);
        if !r_is_valid {
            continue;
        }

        cache.insert(t, true);
        return true;
    }

    cache.insert(t, false);
    false
}

fn count_valid_designs<'a>(
    t: &'a str,
    patterns: &HashSet<&'a str>,
    cache: &mut HashMap<&'a str, u32>,
) -> u32 {
    if let Some(b) = cache.get(t) {
        return *b;
    }

    if patterns.contains(t) {
        return 1;
    }

    let mut c = HashMap::new();
    for i in 1..t.len() {
        let (l, r) = t.split_at(i);
        println!("\nsplit at {}: l={}, r={}", i, l, r);

        if design_is_valid(l, patterns, &mut c) || design_is_valid(r, patterns, &mut c) {
            continue;
        };

        println!("valid");

        // println!(
        //     "l_count({})={}, r_count({})={}",
        //     l, l_is_valid, r, r_is_valid
        // );

        cache.entry(t).and_modify(|e| *e += 1).or_insert(1);
    }

    *cache.get(&t).unwrap_or(&0)
}

fn part2(parsed: &(HashSet<&str>, Vec<&str>)) {
    let (patterns, towels) = parsed;

    let mut cache = HashMap::new();

    let result = towels
        .iter()
        .map(|t| count_valid_designs(t, patterns, &mut cache))
        .sum::<u32>();

    println!("part2: {}", result);
    println!(
        "part2: {:?}",
        cache // .iter()
              // .filter(|(k, _)| towels.contains(*k))
              // .collect::<HashMap<_, _>>()
    );
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day19.sample");
    let (_, parsed) = parse(input)?;
    println!("{:?}", parsed.0);
    part1(&parsed);
    part2(&parsed);
    Ok(())
}
