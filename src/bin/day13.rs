use std::ops;

use glam::{DMat2, DVec2, Mat2, Vec2};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u64};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};

fn parse_num_prefixed<T, Input>(prefix: T) -> impl FnMut(Input) -> nom::IResult<Input, u64>
where
    T: nom::InputLength + Clone,
    Input: nom::InputTake
        + nom::Compare<T>
        + nom::InputIter
        + nom::Slice<ops::RangeFrom<usize>>
        + nom::InputLength,
    <Input as nom::InputIter>::Item: nom::AsChar,
{
    move |input| {
        let (input, _) = tag(prefix.clone())(input)?;

        u64(input)
    }
}

fn parse_buttons(input: &str) -> nom::IResult<&str, Mat2> {
    let (input, _) = tag("Button A: ")(input)?;
    let (input, (a1, b1)) = separated_pair(
        parse_num_prefixed("X+"),
        tag(", "),
        parse_num_prefixed("Y+"),
    )(input)?;
    let (input, _) = line_ending(input)?;

    let (input, _) = tag("Button B: ")(input)?;
    let (input, (a2, b2)) = separated_pair(
        parse_num_prefixed("X+"),
        tag(", "),
        parse_num_prefixed("Y+"),
    )(input)?;
    let (input, _) = line_ending(input)?;

    Ok((
        input,
        Mat2::from_cols(
            Vec2::from_array([a1 as f32, b1 as f32]),
            Vec2::from_array([a2 as f32, b2 as f32]),
        ),
    ))
}

fn parse_prize(input: &str) -> nom::IResult<&str, DVec2> {
    let (input, _) = tag("Prize: ")(input)?;
    let (input, (x, y)) = separated_pair(
        parse_num_prefixed("X="),
        tag(", "),
        parse_num_prefixed("Y="),
    )(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, DVec2::new(x as f64, y as f64)))
}

fn part1(parsed: &[(Mat2, DVec2)]) {
    let result = parsed
        .iter()
        .filter_map(|machine| {
            let a = machine.0;
            let a_det = a.determinant() as f64;
            let v = machine.1;

            // cramer's rule.
            let y = DMat2::from_cols(a.col(0).as_dvec2(), v).determinant() / a_det;
            let x = DMat2::from_cols(v, a.col(1).as_dvec2()).determinant() / a_det;

            if x.fract() == 0.0 && y.fract() == 0.0 {
                // println!("x={}, y={}", x, y);
                Some(x * 3.0 + y * 1.0)
            } else {
                None
            }
        })
        .sum::<f64>();

    println!("part1: {}", result);
}

fn part2(parsed: &[(Mat2, DVec2)]) {
    let result = parsed
        .iter()
        .filter_map(|machine| {
            let a = machine.0;
            let a_det = a.determinant() as f64;
            let v = DVec2::new(
                machine.1.x + 10000000000000f64,
                machine.1.y + 10000000000000f64,
            );

            // cramer's rule.
            let y = DMat2::from_cols(a.col(0).as_dvec2(), v).determinant() / a_det;
            let x = DMat2::from_cols(v, a.col(1).as_dvec2()).determinant() / a_det;

            if x.fract() == 0.0 && y.fract() == 0.0 {
                // println!("x={}, y={}", x, y);
                Some(x * 3.0 + y * 1.0)
            } else {
                None
            }
        })
        .sum::<f64>();

    println!("part2: {}", result);
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day13.input");
    let (_, parsed) = separated_list1(line_ending, tuple((parse_buttons, parse_prize)))(input)?;

    // println!("{:?}", parsed);
    part1(&parsed);
    part2(&parsed);

    Ok(())
}
