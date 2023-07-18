mod days;
mod intcode;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("day not given (day1, day2, ...)");
    match arg.as_str() {
        "day1" => days::day1::run(),
        "day2" => days::day2::run(),
        "day3" => days::day3::run(),
        "day4" => days::day4::run(),
        "day5" => days::day5::run(),
        "day6" => days::day6::run(),
        _ => panic!("invalid day"),
    }
}
