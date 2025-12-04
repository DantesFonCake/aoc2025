use std::io::{BufRead, Cursor, Read, Result};
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day1/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = Rotations;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Result<Self::Output> {
        let rotations = input.0;

        let mut pos = 50;
        let mut res = 0usize;
        for rot in rotations {
            pos += rot;
            pos = pos % 100;

            if pos == 0 {
                res += 1;
            }
        }

        Ok(res)
    }

    fn solve_2(input: Self::Input) -> Result<Self::Output> {
        let rotations = input.0;

        let mut pos = 50;
        let mut res = 0usize;
        for rot in rotations {
            let old_pos = pos;

            // считаем гарантированные полные обороты
            let mut full_rotations = (rot.abs() as usize) / 100usize;
            // убираем уже посчитанные полные обороты
            let new_rot = rot % 100;

            let next_pos = (pos + new_rot) % 100;
            let next_pos = if next_pos < 0 {
                100 + next_pos
            } else {
                next_pos
            };
            if old_pos != 0 && ((new_rot > 0 && old_pos > next_pos) || (new_rot < 0 && old_pos < next_pos)) {
                full_rotations += 1;
            } else if next_pos == 0 {
                full_rotations += 1;
            }
            //println!("{old_pos} -> {rot} = {next_pos}; full {full_rotations}");
            pos = next_pos;

            res += full_rotations;
        }

        Ok(res)
    }
}

struct Rotations(Vec<i32>);

impl TaskInput for Rotations {
    fn read(mut input: impl Read) -> Result<Self> {
        let mut rotations = Vec::new();
        input.read_to_end(&mut rotations)?;
        let rotations = Cursor::new(rotations)
            .lines()
            .map(|line| to_rotation(line.unwrap().as_str()))
            .collect();
        Ok(Rotations(rotations))
    }
}

fn to_rotation(rot: &str) -> i32 {
    match rot.as_bytes() {
        [b'R', num @ ..] => str::from_utf8(num)
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        [b'L', num @ ..] => -str::from_utf8(num)
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        _ => panic!("incorrect input: {rot}"),
    }
}