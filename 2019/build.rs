use std::fs;

fn main() {
    let dir = "./src/days";
    let modrs = dir.to_string() + "/mod.rs";

    let mut mods = fs::read_dir(dir)
        .unwrap()
        .filter_map(|x| x.ok())
        .map(|x| x.path().file_stem().unwrap().to_owned())
        .filter(|x| x != "mod")
        .collect::<Vec<_>>();
    mods.sort();

    fs::write(
        modrs,
        mods.iter()
            .map(|x| format!("pub mod {};", x.to_string_lossy()))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
}
