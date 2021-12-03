extern crate anyhow;
extern crate clap;

use anyhow::Context;
use anyhow::Result as AResult;
use clap::Parser;
use day2::Day2;

use crate::day1::Day1;

mod day1;
mod day2;

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

    fn run(task: Task) -> AResult<Self::Out> {
        match task {
            Task::A => Self::run_a().with_context(|| format!("Running Task A for {}", Self::NAME)),
            Task::B => Self::run_b().with_context(|| format!("Running Task B for {}", Self::NAME)),
        }
    }

    fn run_a() -> AResult<Self::Out>;
    fn run_b() -> AResult<Self::Out>;
}

#[derive(Parser)]
struct App {
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
}

#[derive(Parser)]
enum Task {
    A,
    B,
}

fn main() -> AResult<()> {
    let app = App::parse();

    match app.day {
        Day::One { task } => println!("Result: {}", Day1::run(task)?),
        Day::Two { task } => println!("Result: {}", Day2::run(task)?),
    };

    Ok(())
}
