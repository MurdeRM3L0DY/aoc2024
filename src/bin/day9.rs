use std::iter;

use nom::{
    Parser,
    character::complete::{anychar, line_ending},
    combinator::opt,
    multi::fold_many1,
    sequence::terminated,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    File(u32),
}

#[derive(Debug, Default)]
struct InputData {
    blocks: Vec<Block>,
    free_slots: Vec<(u32, u32)>, // Vec<(slot_idx, slot_size)>
    files: Vec<(u32, u32)>,      // Vec<(file_idx, file_size)>
}

fn parse(input: &str) -> nom::IResult<&str, InputData> {
    fold_many1(
        terminated(
            anychar.map(|c| c.to_digit(10).expect("unable to parse to a digit")),
            opt(line_ending),
        ),
        || (InputData::default(), 0, 0),
        |(mut acc, i, mut id), n| {
            let item = if i % 2 == 0 {
                let prev_slot_idx = acc.free_slots.len().saturating_sub(1);
                if let Some(free_slot) = acc.free_slots.get(prev_slot_idx) {
                    let file_idx = free_slot.0 + free_slot.1;
                    acc.files.push((file_idx, n));
                } else {
                    acc.files.push((i, n));
                }

                let item = Block::File(id);
                id += 1;
                item
            } else {
                let prev_file_idx = acc.files.len().saturating_sub(1);
                if let Some(file) = acc.files.get(prev_file_idx) {
                    let free_slot_idx = file.0 + file.1;
                    acc.free_slots.push((free_slot_idx, n));
                } else {
                    acc.free_slots.push((i, n));
                }

                Block::Free
            };

            acc.blocks.extend(iter::repeat(item).take(n as usize));

            (acc, i + 1, id)
        },
    )
    .map(|(parsed, _, _)| parsed)
    .parse(input)
}

fn part1(parsed: &InputData) {
    // println!("parsed: {:?}, len = {}", parsed, parsed.len());

    let blocks = &parsed.blocks;
    let blocks_len = blocks.len();
    let mut file_idx = {
        let mut file_idx = blocks_len - 1;
        let mut block = blocks[file_idx];
        while block == Block::Free {
            file_idx -= 1;
            block = blocks[file_idx];
        }
        file_idx
    };
    let mut idx = 0;

    let mut result = 0u64;
    while idx < blocks_len {
        if idx > file_idx {
            break;
        }

        match blocks[idx] {
            Block::Free => {
                let Block::File(n) = blocks[file_idx] else {
                    unreachable!()
                };
                result += idx as u64 * n as u64;

                // println!("{} * {}; file_idx={}", idx, n, file_idx);

                loop {
                    file_idx -= 1;
                    if matches!(blocks[file_idx], Block::File(..)) {
                        break;
                    }
                }
            }
            Block::File(n) => {
                result += idx as u64 * n as u64;
                // println!("{} * {}", idx, n);
            }
        }

        idx += 1;
    }

    println!("part1: {}", result);
}

fn part2(mut parsed: InputData) {
    // println!("parsed: {:?}", parsed);

    while let Some(file) = parsed.files.pop() {
        let mut free_slot_idx = 0;
        while free_slot_idx < parsed.free_slots.len() {
            let free_slot = parsed.free_slots[free_slot_idx];

            if free_slot.0 >= file.0 {
                break;
            }

            if free_slot.1 >= file.1 {
                let left_idx = free_slot.0 as usize;
                let right_idx = file.0 as usize;

                // println!(
                //     "{:?} swap with {:?}({:?})",
                //     left_range,
                //     (second_idx..second_idx + file.1 as usize),
                //     parsed.blocks[file.0 as usize],
                // );
                let (blocks_l, blocks_r) = parsed.blocks.split_at_mut(right_idx);
                let left_range = left_idx..left_idx + file.1 as usize;
                let right_range = 0..file.1 as usize;
                blocks_l[left_range].swap_with_slice(&mut blocks_r[right_range]);
                // println!("{:?}\n", parsed.blocks);

                if free_slot.1 == file.1 {
                    parsed.free_slots.remove(free_slot_idx);
                } else {
                    parsed.free_slots[free_slot_idx].0 += file.1;
                    parsed.free_slots[free_slot_idx].1 -= file.1;
                }

                break;
            }

            free_slot_idx += 1;
        }
    }

    let result = parsed
        .blocks
        .iter()
        .enumerate()
        .filter_map(|(i, block)| match block {
            Block::Free => None,
            Block::File(f) => Some(i as u64 * (*f) as u64),
        })
        .sum::<u64>();

    println!("part2: {:?}", result);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day9.input");
    let (_, parsed) = parse(input)?;

    part1(&parsed);
    part2(parsed);
    Ok(())
}
