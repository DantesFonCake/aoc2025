use std::env::args;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, Read};
use std::ops::Deref;
use std::time::Instant;

pub trait Task {
    type Input;
    type Output;

    fn solve_1(input: Self::Input) -> std::io::Result<Self::Output>;
    fn solve_2(input: Self::Input) -> std::io::Result<Self::Output>;
}

pub trait TaskInput
where
    Self: Sized,
{
    fn read(input: impl Read) -> std::io::Result<Self>;

    fn read_from_file(input_path: &str) -> std::io::Result<Self> {
        let file = File::open(input_path)?;
        Self::read(file)
    }
}

pub fn run<T: Task>(input_path: &str) -> std::io::Result<()>
where
    <T as Task>::Input: TaskInput,
    <T as Task>::Output: Debug
{
    let args = args().skip(1).collect::<Vec<_>>();
    let input = T::Input::read_from_file(input_path)?;
    let instant = Instant::now();
    let res = match args.first().map(Deref::deref) {
        Some("1") => T::solve_1(input),
        Some("2") => T::solve_2(input),
        _ => Err(Error::other("Expected args to be 1 or 2")),
    }?;
    let elapsed = instant.elapsed();
    println!("{:?}", res);
    println!("elapsed: {:.10}", elapsed.as_secs_f64());
    Ok(())
}
