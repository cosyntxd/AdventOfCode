use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use super::{executor::ExecutionType, inputs::InputInfo};

pub struct Cli {
    pub execution: ExecutionType,
    pub files: Vec<PathBuf>,
}
impl Cli {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        let first_arg = args.get(1).map(|str| str.to_string()).unwrap_or_default();

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
            let cdir = PathBuf::new();
            args[files_list_start..args.len()]
                .iter()
                .map(|str| cdir.join(str))
                .collect()
        } else {
            vec![]
        };

        if execution == ExecutionType::Helping {
            println!("{}", Self::help_message());
        }

        Self { execution, files }
    }
    pub fn parse_as(self) -> Vec<InputInfo> {
        self.files
            .iter()
            .map(|file| {
                let file = fs::canonicalize(file).unwrap();
                let day_num = file
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();
                let year = file
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();
                InputInfo {
                    year: year.parse().unwrap(),
                    day: day_num.parse().unwrap(),
                }
            })
            .collect()
    }
    pub fn help_message() -> &'static str {
        todo!()
    }
}
