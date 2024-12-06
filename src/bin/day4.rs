#[derive(Debug)]
struct CharMatrix {
    rows: u32,
    cols: u32,
    data: Vec<u8>,
}

impl CharMatrix {
    fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            data: vec![],
        }
    }

    fn byte_at(&self, row: i64, col: i64) -> Option<&u8> {
        if !self.is_index(row, col) {
            None
        } else {
            let index = (row as u32 * self.cols + col as u32) as usize;
            self.data.get(index)
        }
    }

    fn is_index(&self, row: i64, col: i64) -> bool {
        (0..self.rows as i64).contains(&row) && (0..self.cols as i64).contains(&col)
    }

    fn offset_to_coord(&self, off: usize) -> (u32, u32) {
        let off = off as u32;
        let row = off / self.cols;
        let col = off % self.cols;
        (row, col)
    }
}

#[derive(Debug)]
enum Direction {
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
}

impl Direction {
    pub fn as_norm(&self) -> (i8, i8) {
        match self {
            Direction::E => (0, 1),
            Direction::NE => (-1, 1),
            Direction::N => (-1, 0),
            Direction::NW => (-1, -1),
            Direction::W => (0, -1),
            Direction::SW => (1, -1),
            Direction::S => (1, 0),
            Direction::SE => (1, 1),
        }
    }

    // assuming counter closewise rotations
    pub fn flipped_90(&self) -> Self {
        match self {
            Direction::E => Direction::N,
            Direction::NE => Direction::NW,
            Direction::N => Direction::W,
            Direction::NW => Direction::SW,
            Direction::W => Direction::S,
            Direction::SW => Direction::SE,
            Direction::S => Direction::E,
            Direction::SE => Direction::NE,
        }
    }

    pub fn flipped_180(&self) -> Self {
        match self {
            Direction::E => Direction::W,
            Direction::NE => Direction::SW,
            Direction::N => Direction::S,
            Direction::NW => Direction::SE,
            Direction::W => Direction::E,
            Direction::SW => Direction::NE,
            Direction::S => Direction::N,
            Direction::SE => Direction::NW,
        }
    }
}

fn parse(input: &str) -> CharMatrix {
    input.lines().fold(CharMatrix::new(), |mut acc, line| {
        acc.rows += 1;
        if acc.cols == 0 {
            acc.cols = line.len() as u32
        }
        acc.data.extend(line.chars().map(|c| c as u8));
        acc
    })
}

const XMAS_STR: [u8; 4] = [b'X', b'M', b'A', b'S'];
const SAMX_STR: [u8; 4] = [b'S', b'A', b'M', b'X'];

fn xmas_at_dir(m: &CharMatrix, dir: &Direction, off: usize, xmas_str: &[u8]) -> bool {
    let xmas_strlen = xmas_str.len();

    let (row, col) = m.offset_to_coord(off);
    let Some(x) = m.byte_at(row as i64, col as i64) else {
        return false;
    };
    if x != &xmas_str[0] {
        return false;
    }

    let (dy, dx) = dir.as_norm();
    let end_row = row as i64 + ((xmas_strlen - 1) as i64 * dy as i64);
    let end_col = col as i64 + ((xmas_strlen - 1) as i64 * dx as i64);

    let Some(s) = m.byte_at(end_row, end_col) else {
        return false;
    };
    if s != &xmas_str[xmas_strlen - 1] {
        return false;
    }

    let mut row = row as i64;
    let mut col = col as i64;
    xmas_str.iter().all(|c| {
        let same_char = c == m.byte_at(row, col).unwrap();
        row += dy as i64;
        col += dx as i64;
        same_char
    })
}

fn part1(m: &CharMatrix) {
    let all_dirs = [
        Direction::NW,
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
    ];

    let result = (0..m.data.len()).fold(0, |acc, i| {
        let count = all_dirs
            .iter()
            .filter(|dir| xmas_at_dir(m, dir, i, &XMAS_STR))
            .count();

        acc + count
    });

    println!("part1: {}", result);
}

fn part2(m: &CharMatrix) {
    let result = (0..m.data.len())
        .filter(|i| {
            (xmas_at_dir(m, &Direction::SE, *i, &XMAS_STR[1..]) // "MAS" in South East Direction
                || xmas_at_dir(m, &Direction::SE, *i, &SAMX_STR[..3])) // "SAM" in South East Direction
                && (xmas_at_dir(m, &Direction::SW, *i + 2, &SAMX_STR[..3]) // "SAM" in South West Direction
                    || xmas_at_dir(m, &Direction::SW, *i + 2, &XMAS_STR[1..])) // "MAS" in South West Direction
        })
        .count();

    println!("part2: {}", result);
}

fn main() -> anyhow::Result<()> {
    // let input = include_str!("../../inputs/day4.sample");
    let input = include_str!("../../inputs/day4.input");

    let m = parse(input);

    part1(&m);
    part2(&m);

    Ok(())
}
