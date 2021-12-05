extern crate anyhow;
extern crate clap;

use std::io::BufRead;

use anyhow::Context;
use anyhow::Result as AResult;
use clap::Parser;
use day2::Day2;

use crate::day1::Day1;
use crate::day3::Day3;

mod day1;
mod day2;
mod day3;

trait DayTask {
    const NAME: &'static str;
    type Out;

    fn input_file() -> std::io::Result<std::io::BufReader<std::fs::File>> {
        std::fs::File::open(format!("./inputs/{}.txt", Self::NAME))
            .map(|it| std::io::BufReader::new(it))
    }

    fn test_file() -> std::io::Result<std::io::BufReader<std::fs::File>> {
        std::fs::File::open(format!("./inputs/{}.test.txt", Self::NAME))
            .map(|it| std::io::BufReader::new(it))
    }

    fn run(task: Task, test: bool) -> AResult<Self::Out> {
        let lines = if test {
            Self::test_file().context("Getting Test File")?
        } else {
            Self::input_file().context("Getting Input File")?
        }
        .lines()
        .collect::<Result<_, _>>()
        .context("Collecting Input Lines")?;

        match task {
            Task::A => {
                Self::run_a(lines).with_context(|| format!("Running Task A for {}", Self::NAME))
            }
            Task::B => {
                Self::run_b(lines).with_context(|| format!("Running Task B for {}", Self::NAME))
            }
        }
    }

    fn run_a(lines: Vec<String>) -> AResult<Self::Out>;
    fn run_b(lines: Vec<String>) -> AResult<Self::Out>;
}

#[derive(Parser)]
struct App {
    #[clap(short, long)]
    test: bool,
    #[clap(subcommand)]
    day: Day,
}

#[derive(Parser)]
enum Day {
    One {
        #[clap(subcommand)]
        task: Task,
    },

    Two {
        #[clap(subcommand)]
        task: Task,
    },

    Three {
        #[clap(subcommand)]
        task: Task,
    },
}

#[derive(Parser)]
enum Task {
    A,
    B,
}

fn main() -> AResult<()> {
    let app = App::parse();

    match app.day {
        Day::One { task } => println!("Result: {}", Day1::run(task, app.test)?),
        Day::Two { task } => println!("Result: {}", Day2::run(task, app.test)?),
        Day::Three { task } => {
            let (gamma, epsilon) = Day3::run(task, app.test)?;
            println!("Result: {:b}, {:b} : {}", gamma, epsilon, gamma * epsilon);
        }
    };

    Ok(())
}
