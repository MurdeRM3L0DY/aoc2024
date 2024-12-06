use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    self, Parser,
    character::complete::{char, line_ending, u32},
    multi::{many_till, many1, separated_list1},
    sequence::{terminated, tuple},
};

fn parse_bsv(input: &str) -> nom::IResult<&str, (u32, u32)> {
    terminated(
        tuple((u32, char('|'), u32)).map(|(n1, _, n2)| (n1, n2)),
        line_ending,
    )(input)
}

fn parse_csv(input: &str) -> nom::IResult<&str, Vec<u32>> {
    terminated(separated_list1(char(','), u32), line_ending)(input)
}

struct InputData {
    constraints: HashSet<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

fn parse(input: &str) -> nom::IResult<&str, InputData> {
    let (input, bsvs) = many_till(parse_bsv, line_ending)
        .map(|(cs, _)| {
            // let implied_constraints = cs
            //     .iter()
            //     .flat_map(|c0| cs.iter().filter(|c1| c0.1 == c1.0).map(|c1| (c0.0, c1.1)))
            //     .collect::<Vec<_>>();

            // println!("{:?}", implied_constraints);
            // HashSet::from_iter(cs.into_iter().chain(implied_constraints))
            HashSet::from_iter(cs)
        })
        .parse(input)?;

    let (input, csvs) = many1(parse_csv)(input)?;

    Ok((input, InputData {
        constraints: bsvs,
        pages: csvs,
    }))
}

struct PageSuccPairIter<'a> {
    pages: &'a [u32],
    i: usize,
}

impl<'a> PageSuccPairIter<'a> {
    fn new(pages: &'a [u32]) -> Self {
        PageSuccPairIter { pages, i: 0 }
    }
}

impl Iterator for PageSuccPairIter<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.pages.len() {
            return None;
        }

        let i = self.i;
        let pages = self.pages;

        let j = i + 1;
        if j < pages.len() {
            self.i += 1;
            Some((pages[i], pages[j]))
        } else {
            self.i += 1;
            self.next()
        }
    }
}

fn part1(data: &InputData) -> anyhow::Result<()> {
    let result = data
        .pages
        .iter()
        .filter(|page| PageSuccPairIter::new(page).all(|pp| data.constraints.contains(&pp)))
        .map(|page| page[page.len() / 2]) // pages are always odd length
        // .inspect(|pp| println!("{:?}", pp))
        .sum::<u32>();
    println!("part1: {}", result);

    Ok(())
}

fn part2(data: &InputData) -> anyhow::Result<()> {
    let result = data
        .pages
        .iter()
        .filter(|page| PageSuccPairIter::new(page).any(|pp| !data.constraints.contains(&pp)))
        .cloned()
        .map(|mut p| {
            let valid_constraints = p
                .iter()
                .cartesian_product(&p)
                .filter(|(a, b)| a != b && data.constraints.contains(&(**a, **b)))
                .fold(HashMap::new(), |mut acc, c| {
                    acc.entry(*c.0).and_modify(|e| *e += 1).or_insert(1);
                    acc
                });

            p.sort_by(|a, b| {
                valid_constraints
                    .get(b)
                    .unwrap_or(&0)
                    .cmp(valid_constraints.get(a).unwrap_or(&0))
            });

            p
        })
        .map(|page| page[page.len() / 2]) // pages are always odd length
        // .inspect(|pp| println!("{:?}", pp))
        .sum::<u32>();

    println!("part2: {:?}", result);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day5.input");
    // let input = include_str!("../../inputs/day5.sample");
    let (_, data) = parse(input)?;

    part1(&data)?;
    part2(&data)?;

    Ok(())
}
