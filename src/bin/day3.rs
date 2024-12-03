use nom::{
    self, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, i32},
    combinator::value,
    multi::{fold_many1, many_till, many1},
    sequence::{delimited, separated_pair},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Instruction {
    pub fn result(&self) -> Option<i32> {
        match self {
            Instruction::Mul(l, r) => Some(l * r),
            Instruction::Do | Instruction::Dont => None,
        }
    }
}

fn parse_mul(input: &str) -> nom::IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let ops = separated_pair(i32, char(','), i32).map(|(l, r)| Instruction::Mul(l, r));
    delimited(char('('), ops, char(')'))(input)
}

fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((
        value(Instruction::Do, tag("do()")),
        value(Instruction::Dont, tag("don't()")),
        parse_mul,
    ))(input)
}

fn parse_only_instructions(input: &str) -> nom::IResult<&str, Instruction> {
    many_till(anychar, parse_instruction)
        .map(|(_, op)| op)
        .parse(input)
}

fn part1(input: &'static str) -> anyhow::Result<()> {
    let (_, ops) = many1(parse_only_instructions)(input)?;
    let result: i32 = ops.iter().filter_map(|op| op.result()).sum();
    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &'static str) -> anyhow::Result<()> {
    let (_, (_, result)) = fold_many1(
        parse_only_instructions,
        || (Instruction::Do, 0),
        |(action, acc), inst| match inst {
            Instruction::Mul(l, r) => {
                if action == Instruction::Do {
                    (action, acc + l * r)
                } else {
                    (Instruction::Dont, acc)
                }
            }

            Instruction::Do => (Instruction::Do, acc),
            Instruction::Dont => (Instruction::Dont, acc),
        },
    )(input)?;
    println!("part 2: {}", result);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // let input = include_str!("../../inputs/day3.p2.sample");
    let input = include_str!("../../inputs/day3.input");

    part1(input)?;
    part2(input)?;

    Ok(())
}
