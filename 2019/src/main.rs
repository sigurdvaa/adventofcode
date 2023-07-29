mod days;
mod intcode;

fn usage(args: &Vec<String>) {
    eprintln!("Usage: {} <all|dayXX>", args[0]);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage(&args);
    }
    match args[1].as_str() {
        "day01" => days::day01::run(),
        "day02" => days::day02::run(),
        "day03" => days::day03::run(),
        "day04" => days::day04::run(),
        "day05" => days::day05::run(),
        "day06" => days::day06::run(),
        "day07" => days::day07::run(),
        "day08" => days::day08::run(),
        "day09" => days::day09::run(),
        "day10" => days::day10::run(),
        "day11" => days::day11::run(),
        "day12" => days::day12::run(),
        "day13" => days::day13::run(),
        "day14" => days::day14::run(),
        "day15" => days::day15::run(),
        "all" => {
            days::day01::run();
            println!();
            days::day02::run();
            println!();
            days::day03::run();
            println!();
            days::day04::run();
            println!();
            days::day05::run();
            println!();
            days::day06::run();
            println!();
            days::day07::run();
            println!();
            days::day08::run();
            println!();
            days::day09::run();
            println!();
            days::day10::run();
            println!();
            days::day11::run();
            println!();
            days::day12::run();
            println!();
            days::day13::run();
            println!();
            days::day14::run();
            println!();
            days::day15::run();
        }
        _ => usage(&args),
    }
}
