use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    env,
    fs::File,
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
};
use ureq::Agent;

pub struct InputInfo {
    pub year: u16,
    pub day: u8,
}

pub fn fetch_data(all_days: Vec<InputInfo>) {
    let client = ureq::agent();
    let io_lock = Mutex::new(());
    let session_info = env::var("AOC_COOKIE").unwrap();

    let futures = all_days.par_iter().for_each(|entry| {
        if !fetch_data_single(&client, entry, &session_info) {
            let _lock = io_lock.lock().unwrap();
            eprintln!("Could not retrieve input for {}-{}", entry.year, entry.day);
        }
    });
}

fn fetch_data_single(client: &Agent, entry: &InputInfo, session_info: &str) -> bool {
    let name = format!("../data/{}-{}-input.txt", entry.year, entry.day);
    let file_path = Path::new(&name);

    if file_path.exists() {
        return true;
    }

    let url_path = format!(
        "https://adventofcode.com/{}/day/{}/input",
        entry.year, entry.day
    );
    println!("{url_path}");

    match client.put(&url_path).set("header", "value").call() {
        Err(err) => return false,
        Ok(response) => {
            let mut reader = response.into_reader();
            let mut file = File::create(file_path).unwrap();
            std::io::copy(&mut reader, &mut file).unwrap();
        }
    }
    true
}
