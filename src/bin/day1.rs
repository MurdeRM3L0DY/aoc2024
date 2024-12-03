use std::{array, collections::HashMap};

fn main() {
    // let input = include_str!("../../inputs/day1.sample");
    let input = include_str!("../../inputs/day1.input");

    part1(input);
    part2(input);
}

fn parse_input(input: &str) -> [Vec<u32>; 2] {
    let lines = input.trim().lines();

    let r = lines.fold(
        array::from_fn::<Vec<u32>, 2, _>(|_| Vec::new()),
        |mut acc, line| {
            let mut line = line.split_ascii_whitespace();

            let l = line.next().expect("unable to get left str");
            let r = line.next().expect("unable to get right str");

            acc[0].push(
                l.parse::<u32>()
                    .expect("unable to parse left str into a u32"),
            );
            acc[1].push(
                r.parse::<u32>()
                    .expect("unable to parse right str into a u32"),
            );

            acc
        },
    );

    r
}

fn part1(input: &str) {
    let [mut left, mut right] = parse_input(input);

    left.sort();
    right.sort();

    let sum_of_diff: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    println!("part 1: {}", sum_of_diff);
}

fn part2(input: &str) {
    let [left, right] = parse_input(input);

    let right_occur_map = right.into_iter().fold(HashMap::new(), |mut acc, n| {
        acc.entry(n).and_modify(|e| *e += 1).or_insert(1u32);
        acc
    });

    let sum_occur: u32 = left
        .iter()
        .map(|n| n * right_occur_map.get(n).unwrap_or(&0))
        .sum();

    println!("part 2: {}", sum_occur);
}
