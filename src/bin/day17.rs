use std::iter;

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{i32, line_ending, u8, u32, u64},
    combinator::opt,
    multi::{fold_many1, many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
};

fn parse_registers(input: &str) -> nom::IResult<&str, [u64; 3]> {
    let (input, (_, a)) = terminated(tuple((tag("Register A: "), u64)), line_ending)(input)?;
    let (input, (_, b)) = terminated(tuple((tag("Register B: "), u64)), line_ending)(input)?;
    let (input, (_, c)) = terminated(tuple((tag("Register C: "), u64)), line_ending)(input)?;

    Ok((input, [a, b, c]))
}

fn parse_program(input: &str) -> nom::IResult<&str, (Vec<u8>, String)> {
    let (input, _) = tag("Program: ")(input)?;
    let program_str = input.trim().to_string();

    let (input, program) = separated_list1(tag(","), u8)(input)?;

    Ok((input, (program, program_str)))
}
// fn parse_program(input: &str) -> nom::IResult<&str, (Vec<Instruction>, String)> {
//     let (input, _) = tag("Program: ")(input)?;
//     let program_str = input.trim().to_string();
//     let parse_instruction =
//         separated_pair(u8, tag(","), u8).map(|(opcode, raw_operand)| Instruction {
//             opcode: OpCode::from(opcode),
//             arg: raw_operand,
//         });
//
//     let (input, program) = many1(tuple((parse_instruction, opt(tag(",")))).map(|(r, _)| r))(input)?;
//
//     Ok((input, (program, program_str)))
// }

fn parse(input: &str) -> nom::IResult<&str, ([u64; 3], (Vec<u8>, String))> {
    let (input, registers) = parse_registers(input)?;
    let (input, _) = line_ending(input)?;
    let (input, program) = parse_program(input)?;

    Ok((input, (registers, program)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!("skill issue"),
        }
    }
}

#[derive(Debug)]
enum OperandType {
    Literal,
    Combo,
    Ignore,
}

impl OpCode {
    fn operand_type(&self) -> OperandType {
        match self {
            OpCode::Adv => OperandType::Combo,
            OpCode::Bxl => OperandType::Literal,
            OpCode::Bst => OperandType::Combo,
            OpCode::Jnz => OperandType::Literal,
            OpCode::Bxc => OperandType::Ignore,
            OpCode::Out => OperandType::Combo,
            OpCode::Bdv => OperandType::Combo,
            OpCode::Cdv => OperandType::Combo,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: OpCode,
    arg: u8,
}

impl Instruction {
    fn operand(&self, reg: &[u64; 3]) -> u64 {
        match self.opcode.operand_type() {
            OperandType::Literal => self.arg as u64,
            OperandType::Combo => match self.arg {
                n @ 0..=3 => n as u64,
                4 => reg[0],
                5 => reg[1],
                6 => reg[2],
                _ => unreachable!(),
            },
            OperandType::Ignore => u64::MIN,
        }
    }
}

fn part1(parsed: ([u64; 3], (Vec<u8>, String))) {
    let (mut reg, (program, program_str)) = parsed;
    reg[0] = 117440;

    let instructions = program
        .chunks_exact(2)
        .map(|c| {
            let opcode = c[0];
            let arg = c[1];
            Instruction {
                opcode: OpCode::from(opcode),
                arg,
            }
        })
        .collect::<Vec<_>>();

    let mut ip = 0u32;
    let mut out = String::new();

    while ip < instructions.len() as u32 * 2 {
        let instruction = &instructions[(ip / 2) as usize];
        let (opcode, operand) = (&instruction.opcode, instruction.operand(&reg));

        // println!("instruction={:?}", instruction);

        // println!("reg={:?}", reg);
        match opcode {
            OpCode::Adv => {
                let a = &mut reg[0];
                // println!("[ADV] a_pre={:?}, operand={}", a, operand);
                *a /= 2u64.pow(operand as u32);
                // println!("[ADV] a_post={:?}, operand={}", a, operand);

                ip += 2;
            }
            OpCode::Bxl => {
                let b = &mut reg[1];
                *b ^= operand;

                ip += 2;
            }
            OpCode::Bst => {
                let b = &mut reg[1];
                *b = operand % 8;

                ip += 2;
            }
            OpCode::Jnz => {
                let a = &reg[0];
                // println!("a={:?}", a);
                if *a > 0 {
                    ip = operand as u32;
                } else {
                    ip += 2;
                }
            }
            OpCode::Bxc => {
                let c = reg[2];
                let b = &mut reg[1];
                *b ^= c;

                ip += 2;
            }
            OpCode::Out => {
                let c = ((operand % 8) as u8 + 48) as char;
                // println!("operand: {:?}, c: {:?}", operand, c);

                if out.is_empty() {
                    out.push(c);
                } else {
                    out.push(',');
                    out.push(c);
                }

                println!("out={}", out);

                ip += 2;
            }
            OpCode::Bdv => {
                let a = reg[0];
                let b = &mut reg[1];
                *b = a / 2u64.pow(operand as u32);

                ip += 2;
            }
            OpCode::Cdv => {
                let a = reg[0];
                let c = &mut reg[2];
                *c = a / 2u64.pow(operand as u32);

                ip += 2;
            }
        }

        println!("{:?}", reg);
    }

    println!("{:?}", out);
    // println!("{:?}", instructions);

    let mut reg = [0, reg[1], reg[2]];

    // aquine_p2_sample(&program, &mut reg);
    aquine_p2_input(&program, &mut reg);
}

// do
//      a = a >> 3
//      out(a % 8)
// while (a > 0)

// a = 117440;
//   a := 117440 >> 3 -> 14680
//   out(14680 % 8 -> 0)
//
// a = 14680;
//   a := 14680 >> 3 -> 1835
//   out(1835 % 8 -> 3)
//
// a = 1835;
//   a := 1835 >> 3 -> 229
//   out(229 % 8 -> 5)
//
// a = 229;
//   a := 229 >> 3 -> 28
//   out(28 % 8 -> 4)
//
// a = 28;
//   a := 28 >> 3 -> 3
//   out(3 % 8 -> 3)
//
// a = 3;
//   a := 3 >> 3 -> 0
//   out(0 % 8 -> 0)
//
// a = 0;
fn aquine_p2_sample(target: &[u8], reg: &mut [u64]) {
    // (a, b) in iter::zip(0..target.len() - 1, 1..target.len()).rev()

    for a in (0..target.len()).rev() {
        reg[0] = (reg[0] << 3) + target[a] as u64;
        println!("reg[0]={:?}", reg[0]);
    }
    reg[0] <<= 3;

    println!("{:?}", reg);
}

// do
//   b = a % 8
//   b = b ^ 7
//   c = a >> b
//   b = b ^ c
//   b = b ^ 4
//   out(b % 8)
//   a = a >> 3
// while (a > 0)

// do
//   b = (a % 8) ^ 7
//   c = a >> b
//   b = (b ^ c) ^ 4
//   out(b % 8)
//   a = a >> 3
// while (a > 0)

// 31274997412295 < res < 119138258776848

// a=117440; b=0; c=0;  out = "6,1,2,7,4,0"
// b := (117440 % 8) ^ 7 -> 7
// c := 117440 >> 7 -> 917
// b := (7 >> 917) ^ 4 -> 4
// out(4 % 8 -> 4)
// a := 117440 >> 3 -> 14680
//
// a=14680; b=0; c=0;
// b := (14680 % 8) ^ 7 -> 7
// c := 0 >> 7 -> 0
// b := (7 >> 0) ^ 4 -> 3
// out(3 % 8 -> 0)
// a := 117440 >> 3 -> 14680
//
fn aquine_p2_input(target: &[u8], reg: &mut [u64]) {
    for a in (0..target.len()).rev() {
        // reg[1] = (reg[0] % 8) ^ 7;
        // reg[2] = reg[2] << reg[1];
        // reg[1] = ((reg[1] ^ reg[2]) ^ 4 + target[a] as u64);
        // reg[0] = (reg[0] << 3) + (reg[1] % 8);
    }
    // reg[0] <<= 3;

    println!("{:?}", reg);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day17.input");
    let (_, parsed) = parse(input)?;

    // println!("{:?}", parsed);

    part1(parsed);
    Ok(())
}
