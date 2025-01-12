use std::{env, fs, path::Path};

fn main() {
    let output_dir = env::var("OUT_DIR").unwrap();
    let output_modules = Path::new(&output_dir).join("solution_list.rs");
    let watched_dir = Path::new("../");

    for entry in fs::read_dir(watched_dir).unwrap() {
        let entry = entry.unwrap().path();
        if !entry.is_dir() {
            continue;
        }
        let name = entry.file_name().unwrap().to_str().unwrap();

        if !name.chars().all(|c| c.is_ascii_digit()) {
            continue
        }

        println!("cargo:warning=Build script completed successfully. {entry:?}");
    }
    println!("cargo:rerun-if-changed=../");
}