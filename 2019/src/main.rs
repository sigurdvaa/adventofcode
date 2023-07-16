mod days;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("day not given (day1, day2, ...)");
    match arg.as_str() {
        "day1" => days::day1::run(),
        "day2" => days::day2::run(),
        "day3" => days::day3::run(),
        _ => panic!("invalid day"),
    }
}
