use super::DayTask;
use anyhow::{Context, Result as AResult};
use std::io::{stdin, BufRead};

pub struct Day1;

impl DayTask for Day1 {
    const NAME: &'static str = "day1";
    type Out = i32;

    fn run_a() -> AResult<i32> {
        let lines: Vec<String> = Self::input_file()?
            .lines()
            .collect::<Result<_, std::io::Error>>()
            .context("Reading Lines")?;

        let values: Vec<i32> = lines
            .iter()
            .map(|v| v.parse::<i32>())
            .collect::<Result<_, _>>()?;

        Ok(values
            .windows(2)
            .map(|v| v[0] < v[1])
            .filter(|&v| v)
            .count() as i32)
    }
    fn run_b() -> AResult<i32> {
        let lines: Vec<String> = Self::input_file()?
            .lines()
            .collect::<Result<_, std::io::Error>>()
            .context("Reading Lines")?;

        let values: Vec<i32> = lines
            .iter()
            .map(|v| v.parse::<i32>())
            .collect::<Result<_, _>>()?;

        let windowed: Vec<i32> = values.windows(3).map(|b| b.iter().sum()).collect();

        Ok(windowed
            .windows(2)
            .map(|v| v[0] < v[1])
            .filter(|&v| v)
            .count() as i32)
    }
}
