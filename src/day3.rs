use super::DayTask;
use anyhow::Context;
use anyhow::Result as AResult;

pub struct Day3;

impl DayTask for Day3 {
    type Out = (u64, u64);
    const NAME: &'static str = "day3";

    fn run_a(lines: Vec<String>) -> AResult<Self::Out> {
        let cutoff = lines.len();
        let len = lines.get(0).context("There was no input")?.len();

        let counts = lines
            .iter()
            .map(|l| l.chars().map(|c| if c == '1' { 1 } else { 0 }))
            .fold(
                {
                    let mut it = Vec::with_capacity(len);
                    it.resize(len, 0);
                    it
                },
                |mut a, b| {
                    b.enumerate().for_each(|(idx, it)| a[idx] += it);
                    a
                },
            );

        let gamma = counts.iter().enumerate().fold(0, |it, (idx, count)| {
            if *count > cutoff / 2 {
                println!("{}", len - idx);
                it | 1 << (len - idx - 1)
            } else {
                it
            }
        });

        let mask = (0..len).into_iter().fold(0, |acc, _| (acc << 1) | 1);

        let epsilon = !gamma & mask;

        Ok((gamma, epsilon))
    }

    fn run_b(_lines: Vec<String>) -> AResult<Self::Out> {
        todo!()
    }
}
