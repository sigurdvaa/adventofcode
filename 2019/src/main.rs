mod days;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("day not given (day1, day2, ...)");
    match arg.as_str() {
        "day1" => days::day1::run(),
        _ => panic!("invalid day"),
    }
}
