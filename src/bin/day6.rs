use std::collections::{HashMap, HashSet};

enum CharCell {
    Empty,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    E,
    N,
    W,
    S,
}

impl Direction {
    pub fn as_norm(&self) -> (i8, i8) {
        match self {
            Direction::E => (0, 1),
            Direction::N => (-1, 0),
            Direction::W => (0, -1),
            Direction::S => (1, 0),
        }
    }

    pub fn rotate_90_cw(&self) -> Self {
        match self {
            Direction::E => Direction::S,
            Direction::N => Direction::E,
            Direction::W => Direction::N,
            Direction::S => Direction::W,
        }
    }
}

fn parse(input: &str) -> (HashMap<(u32, u32), CharCell>, (u32, u32)) {
    input.lines().enumerate().fold(
        (HashMap::new(), (0, 0)),
        |(mut acc, mut guard_pos), (y, line)| {
            for (x, c) in line.chars().enumerate() {
                let pos = (y as u32, x as u32);
                acc.insert(pos, match c {
                    '#' => CharCell::Obstacle,
                    '^' => {
                        guard_pos = pos;
                        CharCell::Empty
                    }
                    _ => CharCell::Empty,
                });
            }

            (acc, guard_pos)
        },
    )
}

fn part1(data: &HashMap<(u32, u32), CharCell>, mut pos: (u32, u32)) {
    let mut dir = Direction::N;
    let mut count = 0;
    let mut memo = HashSet::new();

    while let Some(c) = data.get(&pos) {
        match c {
            CharCell::Empty => {
                if !memo.contains(&pos) {
                    count += 1;
                    memo.insert(pos);
                }

                let (dy, dx) = dir.as_norm();
                let next_pos = (pos.0 as i64 + dy as i64, pos.1 as i64 + dx as i64);
                if next_pos.0.is_negative() || next_pos.1.is_negative() {
                    break;
                }

                pos = (next_pos.0 as u32, next_pos.1 as u32);
            }
            CharCell::Obstacle => {
                let (dy, dx) = dir.as_norm();
                let prev_pos = (pos.0 as i64 + (-dy) as i64, pos.1 as i64 + (-dx) as i64);

                dir = dir.rotate_90_cw();
                let (dy, dx) = dir.as_norm();
                let next_pos = (prev_pos.0 + dy as i64, prev_pos.1 + dx as i64);
                if next_pos.0.is_negative() || next_pos.1.is_negative() {
                    break;
                }

                pos = (next_pos.0 as u32, next_pos.1 as u32);
            }
        }
    }

    println!("part1: {}", count);
}

fn part2(data: &HashMap<(u32, u32), CharCell>, mut pos: (u32, u32)) {
    let mut dir = Direction::N;
    let mut visited = HashSet::new();
    // let mut obstacles_occupancy = HashSet::new();
    let mut new_obstacles = HashSet::new();

    let mut prev_obstacle = (pos, dir);
    while let Some(c) = data.get(&pos) {
        // println!("current_pos={:?}, current_dir={:?}", pos, dir);
        match c {
            CharCell::Empty => {
                visited.insert(pos);
                // updates the current position
                let (dy, dx) = dir.as_norm();
                let next_pos = (pos.0 as i64 + dy as i64, pos.1 as i64 + dx as i64);
                // if next_pos.0.is_negative() || next_pos.1.is_negative() {
                //     break;
                // }
                let next_pos = (next_pos.0 as u32, next_pos.1 as u32);

                println!(
                    "pos={:?}, obs_pos={:?}, next_maybe_dir={:?}",
                    pos,
                    next_pos,
                    dir.rotate_90_cw()
                );

                if visited.contains(&pos)
                // && obstacles_occupancy.contains(&(pos, dir.rotate_90_cw()))
                {
                    if let Some(CharCell::Empty) = data.get(&next_pos) {
                        pos = prev_obstacle.0;
                        dir = prev_obstacle.1;

                        println!(
                            "prev_obs={:?}, back_pos={:?}, back_dir:{:?}",
                            prev_obstacle, pos, dir
                        );

                        new_obstacles.insert(next_pos);
                        prev_obstacle = (next_pos, dir);
                    }
                } else {
                    pos = next_pos;
                    println!("next_pos={:?}", pos);
                }
            }
            CharCell::Obstacle => {
                println!("pos={:?}, new_dir={:?}", pos, dir);

                // // update neighbours in all directions of the current obstacle
                // for neigh_dir in [Direction::E, Direction::N, Direction::W, Direction::S] {
                //     let mut t_pos = (pos.0 as i64, pos.1 as i64);
                //     let (dy, dx) = neigh_dir.as_norm();
                //
                //     loop {
                //         t_pos = (t_pos.0 + dy as i64, t_pos.1 + dx as i64);
                //         // if t_pos.0.is_negative() || t_pos.1.is_negative() {
                //         //     break;
                //         // }
                //         let t_pos = (t_pos.0 as u32, t_pos.1 as u32);
                //
                //         if let Some(CharCell::Empty) = data.get(&t_pos) {
                //             // starting from the position of the obstacle, it inserts in the hashset:
                //             //  - (pos, Direction::W) for all neighbours `Direction::E` of the obstacle
                //             //  - (pos, Direction::S) for all neighbours `Direction::N` of the obstacle
                //             //  - (pos, Direction::E) for all neighbours `Direction::W` of the obstacle
                //             //  - (pos, Direction::N) for all neighbours `Direction::S` of the obstacle
                //
                //             let neigh_key = (t_pos, neigh_dir.rotate_90_cw().rotate_90_cw());
                //             obstacles_occupancy.insert(neigh_key);
                //         } else {
                //             break;
                //         }
                //     }
                // }

                // gets the previous position just before the obstacle
                let (dy, dx) = dir.as_norm();
                let prev_pos = (pos.0 as i64 + (-dy) as i64, pos.1 as i64 + (-dx) as i64);

                // updates the current direction by rotating it 90 deg clockwise (to the right)
                dir = dir.rotate_90_cw();
                let (dy, dx) = dir.as_norm();

                // updates the current position
                let next_pos = (prev_pos.0 + dy as i64, prev_pos.1 + dx as i64);
                if next_pos.0.is_negative() || next_pos.1.is_negative() {
                    break;
                }
                let next_pos = (next_pos.0 as u32, next_pos.1 as u32);
                pos = next_pos;
            }
        }
    }

    println!("{:?}", new_obstacles);
    println!("part2: {}", new_obstacles.len());

    // println!(
    //     "{}, {}",
    //     obstacles_occupancy.contains(&((0, 6), Direction::W)),
    //     obstacles_occupancy.contains(&((0, 6), Direction::S))
    // );

    // println!(
    //     "{:?}",
    //     obstacles_occupancy.len())
    // );
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day6.sample");

    let (data, pos) = parse(input);
    part1(&data, pos);
    part2(&data, pos);

    Ok(())
}
