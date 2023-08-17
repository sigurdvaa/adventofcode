use std::fs::{read_dir, File};
use std::io::Write;

fn main() {
    let dir = "./src/days";
    println!("cargo:rerun-if-changed={}", dir);
    let mut mod_rs = File::create(dir.to_string() + "/mod.rs").unwrap();

    let mut mods = read_dir(dir)
        .unwrap()
        .filter_map(|x| x.ok())
        .map(|x| x.path().file_stem().unwrap().to_owned())
        .filter(|x| x != "mod")
        .collect::<Vec<_>>();
    mods.sort();

    write!(
        mod_rs,
        "{}\n\
        \n\
        pub const DAYS: &[fn()] = &[\n\
            {}\n\
        ];",
        mods.iter()
            .map(|x| format!("mod {};", x.to_string_lossy()))
            .collect::<Vec<_>>()
            .join("\n"),
        mods.iter()
            .map(|x| format!("    {}::run,", x.to_string_lossy()))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
}
