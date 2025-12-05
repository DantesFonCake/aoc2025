use std::io::{BufRead, BufReader, Read, Result};
use std::ops::RangeInclusive;
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day5/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = FreshDatabase;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Self::Output {
        let count = input
            .ingredients
            .iter()
            .filter(|ingredient| input.fresh.iter().any(|fresh| fresh.contains(*ingredient)))
            .count();

        count
    }

    fn solve_2(input: Self::Input) -> Self::Output {
        let mut fresh_db = input.fresh;
        fresh_db.sort_unstable_by_key(|fresh| *fresh.start());

        let mut count = 0usize;

        let mut prev: Option<RangeInclusive<usize>> = None;

        for fresh in fresh_db {
            if let Some(prev_fresh) = prev.clone() {
                if fresh.start() > prev_fresh.end() {
                    count += prev_fresh.count();
                    prev = Some(fresh);
                } else if fresh.end() > prev_fresh.end() {
                    prev = Some(*prev_fresh.start()..=*fresh.end());
                }
            } else {
                prev = Some(fresh);
            }
        }

        count += prev.map(RangeInclusive::count).unwrap_or(0);
        count
    }
}

struct FreshDatabase {
    fresh: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl TaskInput for FreshDatabase {
    fn read(input: impl Read) -> Result<Self> {
        let reader = BufReader::new(input);
        let mut fresh = vec![];
        let mut ingredients = vec![];

        let mut lines = reader.lines();
        while let Some(line) = lines.next().transpose()? {
            if line.is_empty() {
                break;
            }

            let dash = line.trim().find('-').unwrap();
            let (start, end) = line.split_at(dash);
            let start = start.parse().unwrap();
            let end = end[1..].parse().unwrap();
            fresh.push(start..=end);
        }

        while let Some(line) = lines.next().transpose()? {
            let num = line.trim();
            if num.is_empty() {
                break;
            }

            let num = num.parse().unwrap();
            ingredients.push(num);
        }

        Ok(FreshDatabase { fresh, ingredients })
    }
}
