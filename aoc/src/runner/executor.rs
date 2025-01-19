use std::{collections::{HashMap, HashSet}, time::{Duration, Instant}};

use super::inputs::InputInfo;

const MAX_RUNS: usize = 10_000;
const MIN_RUNS: usize = 5;
const TIMEOUT: Duration = Duration::from_secs(1);

// ew
type SolutionFunc = dyn Fn() -> Box<dyn ToString>;
type NanoSec = u128;
struct ExecutionResults {
    print: HashSet<String>,
    times_ns: Vec<NanoSec>,
}
impl ExecutionResults {
    pub fn run(func: &SolutionFunc) -> Self {
        let mut times_ns = Vec::new();
        let start_time = Instant::now();
        let mut function_results = HashSet::new();
    
        for r in 0..MAX_RUNS {
            if start_time.elapsed() >= TIMEOUT && r > MIN_RUNS {
                break;
            }
    
            let run_start = Instant::now();
            let result_new = func();
            times_ns.push(run_start.elapsed().as_nanos() as NanoSec);

            function_results.insert(result_new.to_string());
        }
        times_ns.sort_unstable();
        Self { print: function_results, times_ns }
    }
    pub fn mean(&self) -> NanoSec {
        self.times_ns.iter().cloned().sum::<NanoSec>() / self.times_ns.len() as NanoSec
    }
    pub fn median(&mut self) -> NanoSec {
        let mid = self.times_ns.len() / 2;
        if self.times_ns.len() % 2 == 0 {
            (self.times_ns[mid - 1] + self.times_ns[mid]) / 2
        } else {
            self.times_ns[mid]
        }
    }
    pub fn sigma(&self) -> f32 {
        let mean = self.mean();
        let n = self.times_ns.len() as f32;
        let error = self.times_ns.iter().map(|t| mean.abs_diff(*t).pow(2)).sum::<u128>() as f32;
        let correction = -1.5 + 1.0 / (8.0 * (n - 1.0));
        (error / (n + correction)).sqrt()
    }
}

pub struct Executor {
    execution_type: ExecutionType,
    files: (Box<SolutionFunc>, InputInfo),
    results: HashMap<InputInfo, ExecutionResults>,
}
impl Executor {
    pub fn new() {

    }
    pub fn run() {

    }
    pub fn pretty_print() {

    }
}


#[derive(PartialEq)]
pub enum ExecutionType {
    Run,
    Bench,
    Helping,
}
