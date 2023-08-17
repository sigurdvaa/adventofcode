mod days;
mod intcode;

fn usage(args: &Vec<String>) {
    eprintln!(
        "Usage: {} {{ all | day1..day{} }}",
        args[0],
        days::DAYS.len()
    );
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage(&args);
    }
    match args[1].as_str() {
        arg if arg.starts_with("day") => match arg[3..].parse::<usize>() {
            Ok(day) if 0 < day && day <= days::DAYS.len() => {
                days::DAYS[day - 1]();
            }
            Err(err) => {
                eprintln!("Invalid day, {err}: {}", &arg[3..]);
            }
            _ => {
                usage(&args);
            }
        },
        "all" => {
            for day in days::DAYS {
                day();
                println!();
            }
        }
        _ => usage(&args),
    }
}
