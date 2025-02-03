use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    path::Path,
    sync::{Arc, Mutex},
};
use ureq::Agent;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputInfo {
    pub year: u16,
    pub day: u8,
}

pub fn fetch_data(all_days: Vec<InputInfo>) -> Arc<Mutex<HashMap<InputInfo, Option<String>>>> {
    let client = ureq::agent();
    let io_lock = Mutex::new(());
    let session_info = env::var("AOC_COOKIE").unwrap();

    let map = Arc::new(Mutex::new(HashMap::new()));

    all_days.par_iter().for_each(|entry| {
        let map = Arc::clone(&map);
        let result = fetch_data_single(&client, entry, &session_info);
        let _lock = io_lock.lock().unwrap();
        if result.is_none() {
            eprintln!("Could not retrieve input for {}-{}", entry.year, entry.day);
        }
        map.lock().unwrap().insert(*entry, result);
    });
    map
}

fn fetch_data_single(client: &Agent, entry: &InputInfo, session_info: &str) -> Option<String> {
    let name = format!("aoc/data/{}-{}-input.txt", entry.year, entry.day);
    let file_path = Path::new(&name);
    // println!("{file_path:?}");
    // return Some(fs::read_to_string(file_path).unwrap());

    if file_path.exists() {
        return fs::read_to_string(file_path).ok();
    } else {
        let abs_path = env::current_dir().unwrap().join(file_path);
        panic!("File not found: {}", abs_path.display());
    }

    let url_path = format!(
        "https://adventofcode.com/{}/day/{}/input",
        entry.year, entry.day
    );
    println!("{url_path}");

    match client.put(&url_path).set("header", "value").call() {
        Err(err) => None,
        Ok(response) => {
            let mut reader = response.into_reader();
            let mut file = File::create(file_path).unwrap();
            let mut return_str = String::new();
            reader.read_to_string(&mut return_str).unwrap();
            std::io::copy(&mut reader, &mut file).unwrap();
            Some(return_str)
        }
    }
}
