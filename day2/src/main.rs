use std::io::{BufRead, BufReader, Read, Result};
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day2/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = Ranges;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Self::Output {
        let ranges = input.0;
        let mut res = 0usize;

        for range in ranges {
            let (start, end) = range;

            let mut id = start;
            while id <= end {
                let len = id.ilog10() + 1;
                if len % 2 != 0 {
                    id = 10usize.pow(len);
                    continue;
                }

                let part_len = len / 2;
                let mask = mask(part_len);
                let (_right, mut left) = parts(id, part_len);
                let mut new_id = left * mask + left;
                while new_id <= end {
                    //print!("{id}: {left} - {right}; {new_id} ");
                    if new_id >= start && left < mask {
                        //print!("- hit!");
                        res += new_id;
                    }

                    //println!();
                    left += 1;
                    new_id = left * mask + left;
                }

                id = new_id;
            }
        }
        res
    }

    fn solve_2(input: Self::Input) -> Self::Output {
        let ranges = input.0;
        let mut res = 0usize;

        for range in ranges {
            let (start, end) = range;

            for id in start..=end {
                let len = id.ilog10() + 1;
                let max_part_len = len / 2;

                for part_len in 1..=max_part_len {
                    if len % part_len != 0 {
                        continue;
                    }

                    let mut matches = true;
                    let part_count = len / part_len;
                    let (mut right, mut left) = parts(id, part_len);

                    for _ in 0..part_count - 1 {
                        let old_right = right;
                        (right, left) = parts(left, part_len);
                        if old_right != right {
                            matches = false;
                            break;
                        }
                    }

                    if right == 0 {
                        continue;
                    }

                    //print!("{start}-{end}: {id}; {right}");
                    if matches {
                        res += id;
                        //println!(" - hit!");
                        break;
                    } else {
                        // println!();
                    }
                }
            }
        }
        res
    }
}

const fn mask(len: u32) -> usize {
    10usize.pow(len)
}

const fn parts(id: usize, len: u32) -> (usize, usize) {
    let mask = mask(len);
    (id % mask, id / mask)
}

struct Ranges(Vec<(usize, usize)>);

impl TaskInput for Ranges {
    fn read(input: impl Read) -> Result<Self> {
        let reader = BufReader::new(input);
        let ranges = reader
            .split(b',')
            .map(|r| {
                let line = r.unwrap();
                let dash = line.iter().position(|&c| c == b'-').unwrap();
                let (first, second) = line.split_at(dash);
                let first = str::from_utf8(first).unwrap().trim().parse().unwrap();
                let second = str::from_utf8(&second[1..]).unwrap().trim().parse().unwrap();

                (first, second)
            })
            .collect();

        Ok(Ranges(ranges))
    }
}
