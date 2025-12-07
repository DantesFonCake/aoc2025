use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Read, Result};
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day7/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = Manifold;
    type Output = usize;

    fn solve_1(mut input: Self::Input) -> Self::Output {
        for row in 0..input.rows {
            for col in 0..input.cols {
                if let Some(Tile::Source) = input.get(row, col) {
                    let res = descent_splits(&mut input, (row + 1, col));
                    //println!("{input:?}");
                    return res;
                }
            }
        }

        unreachable!();
    }

    fn solve_2(mut input: Self::Input) -> Self::Output {
        for row in 0..input.rows {
            for col in 0..input.cols {
                if let Some(Tile::Source) = input.get(row, col) {
                    let res = descent_timelines(&mut input, (row + 1, col), &mut HashMap::new());
                    //println!("{input:?}");
                    return res;
                }
            }
        }

        unreachable!();
    }
}

fn descent_splits(manifold: &mut Manifold, (row, col): (usize, usize)) -> usize {
    for next_row in row..manifold.rows {
        let Some(tile) = manifold.get_mut(next_row, col) else {
            break;
        };
        match tile {
            Tile::Source => unreachable!(),
            Tile::Empty => *tile = Tile::Beam,
            Tile::Beam => break,
            Tile::Splitter => {
                let left = descent_splits(manifold, (next_row, col - 1));
                let right = descent_splits(manifold, (next_row, col + 1));
                return left + right + 1;
            }
        }
    }

    0
}

fn descent_timelines(manifold: &mut Manifold, (row, col): (usize, usize), tile_timelines: &mut HashMap<(usize, usize), usize>) -> usize {
    for next_row in row..manifold.rows {
        let Some(tile) = manifold.get_mut(next_row, col) else {
            break;
        };
        match tile {
            Tile::Source => unreachable!(),
            Tile::Empty => *tile = Tile::Beam,
            Tile::Beam => continue,
            Tile::Splitter => {
                let key = (next_row, col);
                if let Some(timelines) = tile_timelines.get(&key) {
                    return *timelines;
                }

                let left = descent_timelines(manifold, (next_row, col - 1), tile_timelines);
                let right = descent_timelines(manifold, (next_row, col + 1), tile_timelines);
                let timelines = left + right;
                tile_timelines.insert(key, timelines);
                return timelines;
            }
        }
    }

    1
}

struct Manifold {
    map: Vec<Tile>,
    cols: usize,
    rows: usize,
}

impl Manifold {
    fn get(&self, row: usize, col: usize) -> Option<Tile> {
        self.map.get(row * self.cols + col).copied()
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Tile> {
        self.map.get_mut(row * self.cols + col)
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Source,
    Empty,
    Beam,
    Splitter,
}

impl TaskInput for Manifold {
    fn read(input: impl Read) -> Result<Self> {
        let reader = BufReader::new(input);
        let mut cols = 0usize;
        let mut rows = 0usize;
        let mut map = vec![];
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            cols = line.len();
            rows += 1;

            map.extend(line.chars().map(|c| match c {
                'S' => Tile::Source,
                '.' => Tile::Empty,
                '|' => Tile::Beam,
                '^' => Tile::Splitter,
                c => panic!("unexpected tile char {}", c),
            }));
        }

        Ok(Manifold { map, cols, rows })
    }
}

impl Debug for Manifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let tile = self.get(row, col).unwrap();
                let char = match tile {
                    Tile::Source => 'S',
                    Tile::Empty => '.',
                    Tile::Beam => '|',
                    Tile::Splitter => '^',
                };
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
