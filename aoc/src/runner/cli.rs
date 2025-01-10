use std::{env, path::PathBuf};

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
            let cdir = PathBuf::from(&args[0]);
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
