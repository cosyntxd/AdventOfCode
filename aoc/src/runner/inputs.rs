use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT}};
use std::{env, fs::File, io::Write, path::Path, sync::{Arc, Mutex}};

pub struct InputInfo {
    pub year: u16,
    pub day: u8,
}

pub fn fetch_data(all_days: Vec<InputInfo>) {
    let client = reqwest::blocking::Client::new();
    let io_lock = Arc::new(Mutex::new(()));

    let mut headers = HeaderMap::new();

    let session_info = format!("session={}", env::var("AOC_COOKIE").unwrap());
    headers.insert(COOKIE, HeaderValue::from_str(&session_info).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_str("rust-cli").unwrap());

    let futures = all_days.par_iter().map(|entry| {
        if !fetch_data_single(&client, entry, headers.clone()) {
            let _lock = io_lock.lock().unwrap();
            eprintln!("Could not retrieve input for {}-{}", entry.year, entry.day);
        }
    });
}

fn fetch_data_single(client: &Client, entry: &InputInfo, headers: HeaderMap) -> bool {
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
    

    match client
        .get(url_path)
        .headers(headers)
        .send() {
        Ok(response) => {
            if !response.status().is_success() {
                return false;
            }
            if let Ok(text) = response.text() {
                if let Ok(mut file) = File::create(file_path) {
                    if file.write_all(text.as_bytes()).is_ok() {
                        return true;
                    }
                }
            }
        }
        Err(_) => return false,
    }
    true
}