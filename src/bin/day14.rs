use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
};

use glam::{I8Vec2, I16Vec2, U8Vec2};
use nom::{
    bytes::complete::tag,
    character::complete::{i16, line_ending, space1, u8},
    multi::many1,
    sequence::{separated_pair, terminated},
};

fn parse_position(input: &str) -> nom::IResult<&str, U8Vec2> {
    let (input, _) = tag("p=")(input)?;
    let (input, (x, y)) = separated_pair(u8, tag(","), u8)(input)?;
    Ok((input, U8Vec2::new(x, y)))
}

fn parse_velocity(input: &str) -> nom::IResult<&str, I16Vec2> {
    let (input, _) = tag("v=")(input)?;
    let (input, (x, y)) = separated_pair(i16, tag(","), i16)(input)?;
    Ok((input, I16Vec2::new(x, y)))
}

fn parse(input: &str) -> nom::IResult<&str, Vec<(U8Vec2, I16Vec2)>> {
    many1(terminated(
        separated_pair(parse_position, space1, parse_velocity),
        line_ending,
    ))(input)
}

const TILES_WIDTH: u8 = 101;
const MID_WIDTH: u8 = 50;
const TILES_HEIGHT: u8 = 103;
const MID_HEIGHT: u8 = 51;
const TSECS: u8 = 100;

fn part1(parsed: &[(U8Vec2, I16Vec2)]) {
    let quad = parsed.iter().fold([0u32; 4], |mut acc, robot| {
        // let endx = ((((robot.0.x as i16 + robot.1.x * TSECS as i16) % TILES_WIDTH as i16)
        //     + TILES_WIDTH as i16)
        //     % TILES_WIDTH as i16) as u8;
        // let endy = ((((robot.0.y as i16 + robot.1.y * TSECS as i16) % TILES_HEIGHT as i16)
        //     + TILES_HEIGHT as i16)
        //     % TILES_HEIGHT as i16) as u8;

        let grid_size = U8Vec2::new(TILES_WIDTH, TILES_HEIGHT).as_i16vec2();
        let endp = ((((robot.0.as_i16vec2() + robot.1 * TSECS as i16) % grid_size) + grid_size)
            % grid_size)
            .as_u8vec2();

        if (MID_WIDTH + 1..TILES_WIDTH).contains(&endp.x) && (0..MID_HEIGHT).contains(&endp.y) {
            acc[1] += 1;
        } else if (0..MID_WIDTH).contains(&endp.x) && (0..MID_HEIGHT).contains(&endp.y) {
            acc[0] += 1;
        } else if (0..MID_WIDTH).contains(&endp.x)
            && (MID_HEIGHT + 1..TILES_HEIGHT).contains(&endp.y)
        {
            acc[3] += 1;
        } else if (MID_WIDTH + 1..TILES_WIDTH).contains(&endp.x)
            && (MID_HEIGHT + 1..TILES_HEIGHT).contains(&endp.y)
        {
            acc[2] += 1;
        }

        // println!("pos={:?}", robot.0);
        acc
    });

    println!("{:?}", quad.into_iter().reduce(|acc, q| acc * q));
}

fn part2_render(parsed: &[(U8Vec2, I16Vec2)]) {
    let mut m = HashMap::new();
    for t in 0..TSECS + 1 {
        for robot in parsed.iter() {
            // let px = ((((robot.0.x as i16 + robot.1.x * t as i16) % TILES_WIDTH as i16)
            //     + TILES_WIDTH as i16)
            //     % TILES_WIDTH as i16) as u8;
            // let py = ((((robot.0.y as i16 + robot.1.y * t as i16) % TILES_HEIGHT as i16)
            //     + TILES_HEIGHT as i16)
            //     % TILES_HEIGHT as i16) as u8;

            let grid_size = U8Vec2::new(TILES_WIDTH, TILES_HEIGHT).as_i16vec2();
            let np = ((((robot.0.as_i16vec2() + robot.1 * t as i16) % grid_size) + grid_size)
                % grid_size)
                .as_u8vec2();

            m.insert(np, '#');
        }

        println!(
            "_____________________________________________ t={}s __________________________________________________",
            t
        );
        for y in 0..TILES_HEIGHT {
            for x in 0..TILES_WIDTH {
                print!("{}", m.get(&U8Vec2::new(x, y)).unwrap_or(&'.'));
            }
            println!();
        }

        for _ in m.drain() {}
        println!()
    }
}

fn get_connected_iter(
    pos: &U8Vec2,
    robot_pos_set: &HashSet<U8Vec2>,
    regions: &mut HashSet<U8Vec2>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(*pos);

    while let Some(pos) = queue.pop_front() {
        if regions.contains(&pos) {
            continue;
        }

        regions.insert(pos);

        for next_pos in [
            I16Vec2::X,
            I16Vec2::X + I16Vec2::NEG_Y,
            I16Vec2::NEG_Y,
            I16Vec2::NEG_Y + I16Vec2::NEG_X,
            I16Vec2::NEG_X,
            I16Vec2::NEG_X + I16Vec2::Y,
            I16Vec2::Y,
            I16Vec2::Y + I16Vec2::X,
        ]
        .into_iter()
        .filter_map(|dir| {
            let next_pos = pos.as_i16vec2() + dir;
            if next_pos.is_negative_bitmask() > 0 {
                return None;
            }
            let next_pos = next_pos.as_u8vec2();

            robot_pos_set.contains(&next_pos).then_some(next_pos)
        }) {
            queue.push_back(next_pos);
        }
    }
}

fn part2_heu(parsed: &[(U8Vec2, I16Vec2)]) {
    let mut connected = HashSet::new();
    let mut visited = HashSet::new();
    let grid_size = U8Vec2::new(TILES_WIDTH, TILES_HEIGHT).as_i16vec2();

    for t in 0..TSECS + 1 {
        let robot_pos_set = parsed
            .iter()
            .map(|robot| {
                ((((robot.0.as_i16vec2() + robot.1 * t as i16) % grid_size) + grid_size)
                    % grid_size)
                    .as_u8vec2()
            })
            .collect::<HashSet<_>>();

        for pos in robot_pos_set.iter() {
            if visited.contains(pos) {
                continue;
            }

            get_connected_iter(pos, &robot_pos_set, &mut connected);

            visited.extend(connected.drain());
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day14.input");
    let (_, parsed) = parse(input)?;
    // println!("{:?}", parsed);
    part1(&parsed);
    // part2_render(&parsed);
    Ok(())
}
