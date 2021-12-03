mod day1;
mod day2;

fn main() -> Result<(), std::io::Error> {
    //day1::run_part2().map(|v| println!("Increased: {}", v))
    day2::run_part2().map(|v| println!("Final: {}", v))
}
