extern crate anyhow;
extern crate clap;

use anyhow::Context;
use anyhow::Result as AResult;
use clap::Parser;

mod day1;
mod day2;

trait DayTask {
    const NAME: &'static str;
    type Out;

    fn run(task: Task) -> AResult<Self::Out> {
        match task {
            Task::A => Self::run_a(),
            Task::B => Self::run_b(),
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

    let result = match app.day {
        Day::One { task } => unimplemented!(),
        Day::Two { task } => unimplemented!(),
    };
}
