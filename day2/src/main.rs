use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Error, Result};
use std::ops::Deref;
use std::time::Instant;

fn main() -> Result<()> {
    let args = args().skip(1).collect::<Vec<_>>();
    let input_file = "input.txt";
    let res = match args.first().map(Deref::deref) {
        Some("1") => solve_1(input_file),
        Some("2") => solve_2(input_file),
        _ => Err(Error::other("Expected args to be 1 or 2")),
    }?;
    println!("{}", res);
    Ok(())
}

pub(crate) fn solve_1(path: &str) -> Result<usize> {
    let ranges = read_ranges(path)?.collect::<Result<Vec<_>>>()?;
    let mut res = 0usize;

    let instant = Instant::now();
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
    let elapsed = instant.elapsed().as_secs_f64();

    println!("{elapsed:.20}");
    Ok(res)
}

pub(crate) fn solve_2(path: &str) -> Result<usize> {
    let ranges = read_ranges(path)?.collect::<Result<Vec<_>>>()?;
    let mut res = 0usize;

    let instant = Instant::now();
    for range in ranges {
        let (start, end) = range;

        for id in start..=end {
            let len = id.ilog10() + 1;
            let max_part_len = len / 2;

            for part_len in (1..=max_part_len) {
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
    let elapsed = instant.elapsed().as_secs_f64();

    println!("{elapsed:.20}");
    Ok(res)
}

const fn mask(len: u32) -> usize {
    10usize.pow(len)
}

const fn parts(id: usize, len: u32) -> (usize, usize) {
    let mask = mask(len);
    (id % mask, id / mask)
}

fn read_ranges(path: &str) -> Result<impl Iterator<Item = Result<(usize, usize)>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let ranges = reader
        .split(b',')
        .map(|r| r.map(|r| BufRead::split(Cursor::new(r), b'-')))
        .map(|r| {
            let mut r = r?;
            let first = r
                .next()
                .unwrap_or_else(|| Err(Error::other("invalid format")))?;
            let first = str::from_utf8(&first).map_err(Error::other)?.trim();
            let first = first.parse::<usize>().map_err(Error::other)?;
            let second = r
                .next()
                .unwrap_or_else(|| Err(Error::other("invalid format")))?;
            let second = str::from_utf8(&second).map_err(Error::other)?.trim();
            let second = second.parse::<usize>().map_err(Error::other)?;

            Ok((first, second))
        });

    Ok(ranges)
}
