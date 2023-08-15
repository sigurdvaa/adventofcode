mod days;
mod intcode;

const DAYS: &[fn()] = &[
    days::day01::run,
    days::day02::run,
    days::day03::run,
    days::day04::run,
    days::day05::run,
    days::day06::run,
    days::day07::run,
    days::day08::run,
    days::day09::run,
    days::day10::run,
    days::day11::run,
    days::day12::run,
    days::day13::run,
    days::day14::run,
    days::day15::run,
    days::day16::run,
    days::day17::run,
    days::day18::run,
];

fn usage(args: &Vec<String>) {
    eprintln!("Usage: {} {{ all | day1..day{} }}", args[0], DAYS.len());
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage(&args);
    }
    match args[1].as_str() {
        arg if arg.starts_with("day") => match arg[3..].parse::<usize>() {
            Ok(day) if 0 < day && day <= DAYS.len() => {
                DAYS[day - 1]();
            }
            Err(err) => {
                eprintln!("Invalid day, {err}: {}", &arg[3..]);
            }
            _ => {
                usage(&args);
            }
        },
        "all" => {
            for day in DAYS {
                day();
                println!();
            }
        }
        _ => usage(&args),
    }
}
