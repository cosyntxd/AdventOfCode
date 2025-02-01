use std::{collections::HashMap, env, fs::{self, File}, io::Write, path::Path};

fn main() {
    let output_dir = env::var("OUT_DIR").unwrap();
    let output_modules = Path::new(&output_dir).join("solution_list.rs");
    let mut out_file = File::create(output_modules).unwrap();
    let watched_dir = Path::new("src");
    let mut funcs = HashMap::new();
    for entry in fs::read_dir(watched_dir).unwrap() {
        let entry = entry.unwrap().path();
        if !entry.is_dir() {
            continue;
        }
        let name = entry.file_name().unwrap().to_str().unwrap();

        if !name.chars().all(|c| c.is_ascii_digit()) {

            continue;
        }
        out_file.write(format!("mod year_{name} {{ \n").as_bytes());
        for day in fs::read_dir(&entry).unwrap() {
            let day = day.unwrap().path();
            let file_name = day.file_name().unwrap_or_default().to_str().unwrap();
            if !file_name.chars().any(|c| c.is_ascii_digit()) {
                continue;
            }
            let day_num = file_name.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            let inc_way = format!("    pub mod day_{day_num} {{  include!(\"../../../../../aoc/src/2024/day01.rs\");  }} \n");
            let key = format!("year_{name}_day_{day_num}");
            
            funcs.insert(format!("{key}"), format!("year_{name}::day_{day_num}::run"));
            out_file.write(inc_way.as_bytes()).unwrap();

        }
        out_file.write(format!("}} \n").as_bytes());

    }
    out_file.write("use std::collections::HashMap;\n".as_bytes());
    out_file.write("pub fn get_list_gen() -> HashMap<String, Box<fn() -> (u32, u32)>> {\n".as_bytes());
    out_file.write("    let mut map = HashMap::new();\n".as_bytes());
    for (k,v) in funcs.into_iter() {
        out_file.write(format!("    map.insert(\"{k}\".into(),Box::new({v} as fn() -> (u32, u32)));\n").as_bytes());
    }
    out_file.write("    return map;\n".as_bytes());
    out_file.write(format!("}} \n").as_bytes());
    println!("cargo:rerun-if-changed=../");
}