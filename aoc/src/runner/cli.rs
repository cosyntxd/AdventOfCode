use std::{env, fs::{self, DirEntry}, iter, path::{Path, PathBuf, MAIN_SEPARATOR}};

use regex::Regex;

pub struct Cli {
    pub execution: ExecutionType,
    pub files: Vec<PathBuf>,
}
impl Cli {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        let first_arg = args
            .get(1).map(|str| str.to_string())
            .unwrap_or_default();

        let mut keyword = true;
        let execution = match first_arg.to_lowercase().as_str() {
            "help" | "h" | "--help" => ExecutionType::Helping,
            "bench" | "b" => ExecutionType::Bench,
            "run" | "r" => ExecutionType::Run,
            _ => {
                keyword = false;
                ExecutionType::Run
            }
        };

        let files_list_start = match keyword {
            true => 2,
            false => 1,
        };

        let files = if args.len() > 1 {
            let mut cdir = PathBuf::new();
            args[files_list_start..args.len()]
                .iter()
                .map(|str| {
                    cdir.join(str)
                })
                .collect()
        } else {
            vec![]
        };

        if execution == ExecutionType::Helping {
            println!("{}", Self::help_message());
        }

        Self { execution, files }
    }
    pub fn files_parse(&self) -> Vec<PathBuf> {
        let mut good_days = vec![];

        for path in &self.files {
            if path.exists() {
                good_days.push(path.clone());
                continue;
            }
            else if path.is_dir() {
                good_days.extend(Self::get_dir_files(path));
                continue;
            }
            if !path.parent().unwrap().is_dir() {
                eprintln!("who is your father???");
                continue;
            }
            let name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
            if name.contains("*") {
                let path_full = path.as_os_str().to_string_lossy();
                let wildcard_pos = path_full.chars().position(|c| c == '*').unwrap();
                let (before, after) = path_full.split_at(wildcard_pos);


                for entry in Self::get_dir_files(path.parent().unwrap()) {
                    let entry_full = entry.as_os_str().to_string_lossy();
                    
                    if entry_full.starts_with(before) && entry_full.ends_with(after) {
                        if entry.is_dir() {
                            good_days.extend(Self::get_dir_files(&entry));
                        } else if entry.is_file() {
                            good_days.push(entry);
                        } else {
                            panic!("boy wtf")
                        }
                    }
                }
            }
        }
        good_days
    }
    fn get_dir_files(path: &Path) -> Box<dyn Iterator<Item = PathBuf>> {
        match fs::read_dir(path) {
            Err(_) => Box::new(iter::empty()),
            Ok(entries) => Box::new(entries.filter_map(Result::ok).map(|e| e.path())),
        }
    }
    pub fn help_message() -> &'static str {
        todo!()
    }
}
#[derive(PartialEq)]
pub enum ExecutionType {
    Run,
    Bench,
    Helping,
}
