mod runner;
use runner::{cli, executor, inputs};
pub fn main() {
    let args = cli::Cli::parse();
    for f in args.files {
        println!("{f:?}");
    }

    // let rt = tokio::runtime::Builder::new_multi_thread()
    // .worker_threads(20)  // Customize the number of worker threads
    // .enable_all()
    // .build()
    // .unwrap();
    // Use the runtime to run the async function
    // let mut a = vec![];
    // for y in 2021..2025 {
    //     for j in 1..20 {
    //         a.push(InputInfo { year: y, day: j })
    //     }
    // }
    // fetch_data(a);
    // match args.command {
    //     cli::ExecutionType::Run {  } => todo!(),
    //     cli::ExecutionType::Bench {  } => todo!(),
    // }
}
