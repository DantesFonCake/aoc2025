use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Read, Result};
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day4/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = Floor;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Result<Self::Output> {
        let mut count = 0usize;
        //println!("{input:?}");
        for x in 0..input.cols {
            for y in 0..input.rows {
                if let Some(tile) = input.get(x, y)
                    && tile == 1
                {
                    let neighborhood = read_mask((x, y), &input, MASK3);
                    let rolls = (neighborhood.iter().copied().sum::<u8>() as usize) - 1usize;
                    if rolls < 4 {
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }

    fn solve_2(mut input: Self::Input) -> Result<Self::Output> {
        let mut count = 0usize;
        loop {
            let old_count = count;
            //println!("{input:?}");
            for x in 0..input.cols {
                for y in 0..input.rows {
                    if let Some(tile) = input.get(x, y)
                        && tile == 1
                    {
                        let neighborhood = read_mask((x, y), &input, MASK3);
                        let rolls = (neighborhood.iter().copied().sum::<u8>() as usize) - 1usize;
                        if rolls < 4 {
                            count += 1;
                            input.set(x, y, 0);

                        }
                    }
                }
            }

            if old_count == count {
                break;
            }
        }
        Ok(count)
    }
}

fn read_mask<const N: usize>(
    origin: (usize, usize),
    map: &Floor,
    mask: [(i32, i32); N],
) -> [u8; N] {
    mask.map(|(x_offset, y_offset)| {
        let x = origin.0 as i32 + x_offset;
        let y = origin.1 as i32 + y_offset;

        if x < 0 || y < 0 || x as usize >= map.cols || y as usize >= map.rows {
            None
        } else {
            map.get(x as usize, y as usize)
        }
        .unwrap_or(0u8)
    })
}

const MASK3: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct Floor {
    cols: usize,
    rows: usize,
    map: Vec<u8>,
}

impl Floor {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.map.get(x + y * self.cols).copied()
    }

    fn set(&mut self, x: usize, y: usize, tile: u8) {
        self.map[x + y * self.cols] = tile;
    }
}

impl Debug for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                let tile = self.get(x, y).unwrap();
                write!(f, "{}", if tile == 0 { '.' } else { '@' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl TaskInput for Floor {
    fn read(input: impl Read) -> Result<Self> {
        let mut res = vec![];
        let lines = BufReader::new(input).lines();
        let mut rows = 0usize;
        let mut cols = 0usize;
        for line in lines {
            rows += 1;
            let line = line?;
            let line = line
                .trim()
                .chars()
                .map(|c| match c {
                    '.' => 0u8,
                    '@' => 1u8,
                    c => panic!("unexpected character '{}'", c),
                })
                .collect::<Vec<_>>();
            res.extend(&line);
            cols = line.len();
        }

        Ok(Floor {
            cols,
            rows,
            map: res,
        })
    }
}
