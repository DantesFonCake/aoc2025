use std::{
    fs::File,
    io::{BufRead, Cursor, Error, Read, Result},
};

pub fn solve_1(path: &str) -> Result<usize> {
    let rotations = read_rotations_from_file(path)?;

    let mut pos = 50;
    let mut res = 0usize;
    for rot in rotations {
        let rot = rot?;
        pos += rot;
        pos = pos % 100;

        if pos == 0 {
            res += 1;
        }
    }

    Ok(res)
}

pub fn solve_2(path: &str) -> Result<usize> {
    let rotations = read_rotations_from_file(path)?;

    let mut pos = 50;
    let mut res = 0usize;
    for rot in rotations {
        let rot = rot?;
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
        println!("{old_pos} -> {rot} = {next_pos}; full {full_rotations}");
        pos = next_pos;

        res += full_rotations;
    }

    Ok(res)
}

fn read_rotations_from_file(path: &str) -> Result<impl Iterator<Item = Result<i32>>> {
    let mut file = File::open(path)?;
    let mut rotations = Vec::new();
    file.read_to_end(&mut rotations)?;
    let rotations = Cursor::new(rotations)
        .lines()
        .map(|line| to_rotation(line?.as_str()));
    Ok(rotations)
}

fn to_rotation(rot: &str) -> Result<i32> {
    let rot = match rot.as_bytes() {
        [b'R', num @ ..] => str::from_utf8(num)
            .map_err(|e| Error::other(e.to_string()))?
            .parse::<i32>()
            .map_err(|e| Error::other(e.to_string()))?,
        [b'L', num @ ..] => -str::from_utf8(num)
            .map_err(|e| Error::other(e.to_string()))?
            .parse::<i32>()
            .map_err(|e| Error::other(e.to_string()))?,
        _ => panic!("incorrect input: {rot}"),
    };
    Ok(rot)
}