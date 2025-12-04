use std::io::{BufRead, BufReader, Read, Result};
use util;
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day3/src/input.txt")
}

struct Task;

impl util:: Task for Task {
    type Input = Banks;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Result<Self::Output> {
        let banks = input.0;
        let mut res = 0usize;
        for bank in banks {
            let left = 0usize;
            let right = bank.len() - 1;
            let left_possible = &bank[left..right];
            let (left, left_val) = left_possible
                .iter()
                .copied()
                .enumerate()
                .rev()
                .max_by_key(|(_, val)| *val)
                .unwrap();
            let right_possible = &bank[(left + 1)..];
            let right_val = right_possible.iter().copied().max().unwrap();

            res += (left_val as usize) * 10 + (right_val as usize);
        }

        Ok(res)
    }

    fn solve_2(input: Self::Input) -> Result<Self::Output> {
        let banks = input.0;
        let mut res = 0usize;
        for bank in banks {
            let bank_len = bank.len();
            let mut positions = [
                -1i32,
                (bank_len - 12) as i32,
                (bank_len - 11) as i32,
                (bank_len - 10) as i32,
                (bank_len - 9) as i32,
                (bank_len - 8) as i32,
                (bank_len - 7) as i32,
                (bank_len - 6) as i32,
                (bank_len - 5) as i32,
                (bank_len - 4) as i32,
                (bank_len - 3) as i32,
                (bank_len - 2) as i32,
                (bank_len - 1) as i32,
            ];
            let mut values = [0usize; 12];
            for i in 1..positions.len() {
                let start = positions[i - 1];
                let start = (start + 1) as usize;
                let end = &mut positions[i];
                let search_space = &bank[start..=(*end as usize)];
                let (new_end, val) = search_space
                    .iter()
                    .copied()
                    .enumerate()
                    .rev()
                    .max_by_key(|(_, val)| *val)
                    .unwrap();
                *end = (start + new_end) as i32;
                values[i - 1] = val as usize;
            }

            res += values.iter().fold(0usize, |acc, x| acc * 10 + *x);
        }

        Ok(res)
    }
}

struct Banks(Vec<Vec<u8>>);

impl TaskInput for Banks {
    fn read(input: impl Read) -> Result<Self> {
        let mut res = vec![];
        for line in BufReader::new(input).lines() {
            let line = line?;
            let bank = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            res.push(bank);
        }

        Ok(Banks(res))
    }
}