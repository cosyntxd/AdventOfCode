
use inputs::{fetch_data, InputInfo};


// mod cli;
mod executor;
mod inputs;
pub fn main() {
    for i in 0..2 {
        
    }
    for
    // run();
    // println!("{:?}", env::vars());
    // let args = cli::Cli::parse();

    // // let mut good_days = vec![];
    // for path in args.files {
    //     // if path.exists() {
    //     //     good_days.push(path);
    //     // }
    //     println!("{path:?}");
    // }
    // let rt = tokio::runtime::Builder::new_multi_thread()
    // .worker_threads(20)  // Customize the number of worker threads
    // .enable_all()
    // .build()
    // .unwrap();
    // Use the runtime to run the async function
    let mut a = vec![];
    for y in 2021..2025 {
        for j in 1..20 {
            a.push(InputInfo { year: y, day: j })
        }
    }
    fetch_data(a);
    // match args.command {
    //     cli::ExecutionType::Run {  } => todo!(),
    //     cli::ExecutionType::Bench {  } => todo!(),
    // }
}
