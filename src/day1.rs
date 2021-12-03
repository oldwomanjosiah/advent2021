use std::io::{stdin, BufRead};

pub fn run() -> Result<i32, std::io::Error> {
    let lines: Result<Vec<String>, std::io::Error> = stdin().lock().lines().collect();

    lines.map(|lines| {
        let values: Vec<i32> = lines
            .iter()
            .map(|v| v.parse::<i32>().expect("Could not parse depth as i32"))
            .collect();

        values
            .windows(2)
            .map(|v| v[0] < v[1])
            .filter(|&v| v)
            .count() as i32
    })
}

pub fn run_part2() -> Result<i32, std::io::Error> {
    let lines: Result<Vec<String>, std::io::Error> = stdin().lock().lines().collect();

    lines.map(|lines| {
        let values: Vec<i32> = lines
            .iter()
            .map(|v| v.parse().expect("Could not parse depth as i32"))
            .collect();

        let windowed: Vec<i32> = values.windows(3).map(|b| b.iter().sum()).collect();

        windowed
            .windows(2)
            .map(|v| v[0] < v[1])
            .filter(|&v| v)
            .count() as i32
    })
}
