mod day1;

fn main() -> Result<(), std::io::Error> {
    day1::run_part2().map(|v| println!("Increased: {}", v))
}
