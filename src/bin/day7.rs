use nom::{
    self, Parser,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending, u32, u64},
    multi::{many_till, many1, separated_list1},
    sequence::{terminated, tuple},
};

fn parse_line(input: &str) -> nom::IResult<&str, (u64, Vec<u32>)> {
    terminated(
        tuple((u64, tag(": "), separated_list1(char(' '), u32))),
        line_ending,
    )
    .map(|(target, _, operands)| (target, operands))
    .parse(input)
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Concat,
}

impl Op {
    fn result(&self, a: u64, b: u32) -> u64 {
        match self {
            Op::Mul => a * b as u64,
            Op::Add => a + b as u64,
            Op::Concat => a * (10u64.pow(b.ilog10() + 1)) + b as u64,
        }
    }
}

// NOTE: Alternative: `multi_cartesian_product()` from `Itertools`
fn ops_n_tuple_combinations(n: u32, ops: &[Op]) -> impl Iterator<Item = impl Iterator<Item = Op>> {
    (0..ops.len().pow(n)).map(move |i| {
        (0..n).map(move |j| {
            let index = (i / ops.len().pow(n - 1 - j)) % ops.len();
            ops[index]
        })
    })
}

fn get_composable(target: u64, operands: &[u32], ops: &[Op]) -> Option<u64> {
    ops_n_tuple_combinations(operands.len() as u32 - 1, ops)
        .any(|ops| {
            // NOTE: Alternative: `reduce()` but I wouldn't be able
            // to short circuit
            let mut res = operands[0] as u64;
            for (i, op) in ops.into_iter().enumerate() {
                res = op.result(res, operands[i + 1]);

                if res > target {
                    return false;
                }
            }

            res == target
        })
        .then_some(target)
}

fn part1(data: &[(u64, Vec<u32>)]) {
    let result = data
        .iter()
        .filter_map(|(target, operands)| get_composable(*target, operands, &[Op::Add, Op::Mul]))
        .sum::<u64>();

    println!("part1: {}", result);
}

fn part2(data: &[(u64, Vec<u32>)]) {
    let result = data
        .iter()
        .filter_map(|(target, operands)| {
            get_composable(*target, operands, &[Op::Add, Op::Mul, Op::Concat])
        })
        .sum::<u64>();

    println!("part2: {}", result);
}
fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day7.input");
    let (_, data) = many1(parse_line)(input)?;

    part1(&data);
    part2(&data);

    Ok(())
}
