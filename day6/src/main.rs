use std::io::{BufRead, Cursor, Read, Result};
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day6/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = MathSheet;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Self::Output {
        let mut problems = vec![];
        let mut operations = vec![];
        for line in Cursor::new(input.0).lines() {
            let line = line.unwrap();
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let mut items = line.split(' ').filter(|s| !s.is_empty()).peekable();

            let first = items.peek().unwrap();
            if first.contains(|c| matches!(c, '*' | '+')) {
                parse_line_as_operations(&mut items, &mut operations);
            } else {
                parse_line_as_numbers(&mut items, &mut problems);
            }
        }

        problems
            .into_iter()
            .zip(operations.into_iter())
            .map(|p| match p.1 {
                Operation::Add => p.0.iter().sum::<usize>(),
                Operation::Product => p.0.iter().product(),
            })
            .sum()
    }

    fn solve_2(input: Self::Input) -> Self::Output {
        let homework: MathHomework = input.into();

        let mut res = 0usize;
        let mut problem = vec![];
        let mut cols = (0..homework.cols - 1).rev();
        'cols: while let Some(col) = cols.next() {
            let mut num = 0usize;
            for row in 0..homework.rows {
                match homework.get(row, col) {
                    val @ b'0'..=b'9' => num = num * 10 + (val - b'0') as usize,
                    b' ' => continue,
                    op @ (b'+' | b'*') => {
                        res += match op {
                            b'+' => problem.iter().sum::<usize>() + num,
                            b'*' => problem.iter().product::<usize>() * num,
                            _ => unreachable!("by outer match arm"),
                        };
                        problem.clear();
                        _ = cols.next();
                        continue 'cols;
                    }
                    c => panic!("Unexpected char {}", char::from(c)),
                }
            }
            problem.push(num);
        }

        res
    }
}

struct MathHomework {
    sheet: MathSheet,
    cols: usize,
    rows: usize,
}

impl MathHomework {
    fn get(&self, row: usize, col: usize) -> u8 {
        self.sheet.0.get(row * self.cols + col).copied().unwrap()
    }
}

impl From<MathSheet> for MathHomework {
    fn from(value: MathSheet) -> Self {
        let line_end = value.0.iter().position(|c| *c == b'\n').unwrap();
        let cols = line_end + 1;
        let rows = value.0.len() / cols;

        MathHomework {
            sheet: value,
            cols,
            rows,
        }
    }
}

fn parse_line_as_operations<'a, 'b>(
    line: &'a mut impl Iterator<Item = &'a str>,
    operations: &'b mut Vec<Operation>,
) {
    for op in line {
        match op {
            "+" => operations.push(Operation::Add),
            "*" => operations.push(Operation::Product),
            _ => panic!("Unexpected operation {op}"),
        }
    }
}

fn parse_line_as_numbers<'a, 'b>(
    line: &'a mut impl Iterator<Item = &'a str>,
    problems: &mut Vec<Vec<usize>>,
) {
    let mut pos = 0usize;
    for num in line {
        let problem = if let Some(problem) = problems.get_mut(pos) {
            problem
        } else {
            problems.push(vec![]);
            problems.get_mut(pos).unwrap()
        };
        problem.push(num.parse::<usize>().unwrap());
        pos += 1;
    }
}

struct MathSheet(Vec<u8>);

enum Operation {
    Add,
    Product,
}

impl TaskInput for MathSheet {
    fn read(mut input: impl Read) -> Result<Self> {
        let mut res = vec![];
        input.read_to_end(&mut res)?;
        Ok(MathSheet(res))
    }
}
