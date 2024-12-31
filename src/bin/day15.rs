use std::collections::HashMap;

use glam::{I16Vec2, U8Vec2};
use nom::{
    bytes::complete::{is_not, take_while},
    character::complete::{anychar, line_ending},
    combinator::map,
    multi::{fold_many1, many_till, many1, many1_count},
    sequence::{separated_pair, tuple},
};

#[derive(Debug)]
struct Parsed {
    map: HashMap<U8Vec2, char>,
    start_pos: U8Vec2,
    directions: Vec<I16Vec2>,
}

fn parse_map(input: &str) -> nom::IResult<&str, (HashMap<U8Vec2, char>, U8Vec2)> {
    map(
        fold_many1(
            tuple((is_not("\n"), line_ending)),
            || (HashMap::new(), 0, U8Vec2::MAX),
            |mut acc, (line, _): (&str, &str)| {
                for (x, c) in line.chars().enumerate() {
                    let pos = U8Vec2::new(x as u8, acc.1 as u8);
                    acc.0.insert(pos, c);

                    if c == '@' {
                        acc.2 = pos;
                    }
                }
                acc.1 += 1;
                acc
            },
        ),
        |r| (r.0, r.2),
    )(input)
}

fn parse_directions(input: &str) -> nom::IResult<&str, Vec<I16Vec2>> {
    fold_many1(
        tuple((is_not("\n"), line_ending)),
        Vec::new,
        |mut acc, (line, _): (&str, &str)| {
            for c in line.chars() {
                acc.push(match c {
                    '>' => I16Vec2::X,
                    '^' => I16Vec2::NEG_Y,
                    '<' => I16Vec2::NEG_X,
                    'v' => I16Vec2::Y,
                    _ => unreachable!(),
                });
            }
            acc
        },
    )(input)
}

fn parse(input: &str) -> nom::IResult<&str, Parsed> {
    map(
        separated_pair(parse_map, line_ending, parse_directions),
        |((map, start_pos), directions)| Parsed {
            map,
            directions,
            start_pos,
        },
    )(input)
}

fn move_all_in_dir(map: &mut HashMap<U8Vec2, char>, start_pos: U8Vec2, dir: I16Vec2) -> bool {
    let mut next_pos = start_pos.as_i16vec2() + dir;
    let mut to_move = 1;
    loop {
        if next_pos.is_negative_bitmask() > 0 {
            break;
        }

        if map.get(&next_pos.as_u8vec2()).is_some_and(|c| c != &'O') {
            break;
        }

        to_move += 1;
        next_pos += dir;
    }

    if map.get(&next_pos.as_u8vec2()).is_some_and(|c| c == &'.') {
        for i in 0..to_move {
            let pos = (start_pos.as_i16vec2() + dir) + (i * dir);
            assert!(pos.is_negative_bitmask() == 0, "skill issue");
            let pos = pos.as_u8vec2();
            // println!(
            //     "({:?} + {:?}) + ({:?} * {:?}) = {:?}",
            //     start_pos.as_i16vec2(),
            //     dir,
            //     i,
            //     dir,
            //     pos,
            // );
            map.entry(pos).and_modify(|c| *c = 'O');
        }

        true
    } else {
        false
    }
}

fn part1(mut parsed: Parsed) {
    let mut pos = parsed.start_pos;
    println!("start_pos={:?}", pos);
    // println!("directions={:?}", parsed.directions);
    // println!(
    //     "blocks={:?}",
    //     parsed
    //         .map
    //         .iter()
    //         .filter(|(k, v)| v == &&b'#')
    //         .collect::<Vec<_>>()
    // );
    //
    for dir in parsed.directions.into_iter() {
        // println!("dir={:?}", dir);
        let next_pos = pos.as_i16vec2() + dir;
        if next_pos.is_negative_bitmask() > 0 {
            continue;
        }
        let next_pos = next_pos.as_u8vec2();

        let nc = parsed.map.get(&next_pos);
        // println!("next_pos={:?}, c={:?}", next_pos, nc);
        if nc.is_some_and(|c| c == &'#') {
            // println!("[no move] current_pos={:?}", pos);
            continue;
        } else if nc.is_some_and(|c| c == &'O') {
            if move_all_in_dir(&mut parsed.map, next_pos, dir) {
                parsed.map.entry(pos).and_modify(|c| *c = '.');
                // println!("[moved] current_pos={:?}, next_pos={:?}", pos, next_pos);
                parsed.map.entry(next_pos).and_modify(|c| *c = '@');
                pos = next_pos;
            }
        } else if nc.is_some_and(|c| c == &'.') {
            parsed.map.entry(pos).and_modify(|c| *c = '.');
            // println!("[advanced forward 1] next_pos={:?}", next_pos);
            parsed.map.entry(next_pos).and_modify(|c| *c = '@');
            pos = next_pos;
        }

        // println!()
    }

    // println!("{:?}", parsed.map);

    let result = parsed
        .map
        .into_iter()
        .filter_map(|(k, v)| (v == 'O').then_some(k))
        .map(|pos| 100 * pos.y as u64 + pos.x as u64)
        .sum::<u64>();

    println!("part1: {}", result);
}

fn part2(mut parsed: Parsed) {
    let mut pos = parsed.start_pos;
    println!("start_pos={:?}", pos);
    // println!("directions={:?}", parsed.directions);
    // println!(
    //     "blocks={:?}",
    //     parsed
    //         .map
    //         .iter()
    //         .filter(|(k, v)| v == &&b'#')
    //         .collect::<Vec<_>>()
    // );
    //
    for dir in parsed.directions.into_iter() {
        // println!("dir={:?}", dir);
        let next_pos = pos.as_i16vec2() + dir;
        if next_pos.is_negative_bitmask() > 0 {
            continue;
        }
        let next_pos = next_pos.as_u8vec2();

        let nc = parsed.map.get(&next_pos);
        // println!("next_pos={:?}, c={:?}", next_pos, nc);
        if nc.is_some_and(|c| c == &'#') {
            // println!("[no move] current_pos={:?}", pos);
            continue;
        } else if nc.is_some_and(|c| c == &'O') {
            if move_all_in_dir(&mut parsed.map, next_pos, dir) {
                parsed.map.entry(pos).and_modify(|c| *c = '.');
                // println!("[moved] current_pos={:?}, next_pos={:?}", pos, next_pos);
                parsed.map.entry(next_pos).and_modify(|c| *c = '@');
                pos = next_pos;
            }
        } else if nc.is_some_and(|c| c == &'.') {
            parsed.map.entry(pos).and_modify(|c| *c = '.');
            // println!("[advanced forward 1] next_pos={:?}", next_pos);
            parsed.map.entry(next_pos).and_modify(|c| *c = '@');
            pos = next_pos;
        }

        // println!()
    }

    // println!("{:?}", parsed.map);

    let result = parsed
        .map
        .into_iter()
        .filter_map(|(k, v)| (v == 'O').then_some(k))
        .map(|pos| 100 * pos.y as u64 + pos.x as u64)
        .sum::<u64>();

    println!("part2: {}", result);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day15.input");
    let (_, parsed) = parse(input)?;
    part1(parsed);
    let (_, parsed) = parse(input)?;
    part2(parsed);
    Ok(())
}
