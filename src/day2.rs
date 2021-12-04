use std::io::{stdin, BufRead};

use crate::DayTask;

pub struct Day2;

// TODO(Josiah) Rewrite to be able to return parse errors (instead of panicing) using try_fold
impl DayTask for Day2 {
    const NAME: &'static str = "day2";
    type Out = i32;

    fn run_a() -> anyhow::Result<Self::Out> {
        let lines: Vec<String> = Self::input_file()?
            .lines()
            .collect::<Result<_, std::io::Error>>()?;

        let (vert, hor) = lines
            .iter()
            .map(|l| {
                let tokens: Vec<&str> = l.split(' ').collect();
                assert!(tokens.len() == 2);

                match tokens[0] {
                    "forward" => (
                        0,
                        tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                    ),
                    "backward" => (
                        0,
                        -tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                    ),
                    "up" => (
                        -tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                        0,
                    ),
                    "down" => (
                        tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                        0,
                    ),
                    _ => unreachable!("Unknown Comamnd Recieved"),
                }
            })
            .reduce(|(a, b), (c, d)| (a + c, b + d))
            .unwrap_or((0, 0));

        Ok(vert * hor)
    }

    fn run_b() -> anyhow::Result<Self::Out> {
        let lines: Vec<String> = Self::input_file()?
            .lines()
            .collect::<Result<_, std::io::Error>>()?;

        let (_, vert, hor) = lines
            .iter()
            .map(|l| {
                let tokens: Vec<&str> = l.split(' ').collect();
                assert!(tokens.len() == 2);

                match tokens[0] {
                    "forward" => (
                        0,
                        tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                    ),
                    "backward" => (
                        0,
                        -tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                    ),
                    "up" => (
                        -tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                        0,
                    ),
                    "down" => (
                        tokens[1]
                            .parse::<i32>()
                            .expect("Could not parse token as magnitude"),
                        0,
                    ),
                    _ => unreachable!("Unknown Comamnd Recieved"),
                }
            })
            .fold((0, 0, 0), |(aim, x, depth), (v_change, h_change)| {
                (aim + v_change, x + h_change, depth + (aim * h_change))
            });

        Ok(vert * hor)
    }
}
