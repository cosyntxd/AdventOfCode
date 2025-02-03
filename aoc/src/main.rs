mod runner;

use runner::{
    cli,
    executor::{DaySolution, ExecuteAllSolutions, ExecutionType},
};
pub fn main() {
    let files = cli::Cli::parse().parse_as();
    let solution = files
        .iter()
        .map(|f| DaySolution::new(*f))
        .collect::<Vec<_>>();
    let executor = ExecuteAllSolutions::new(solution, ExecutionType::Run);
    executor.run();
}
