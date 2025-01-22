use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use ureq::rustls::crypto::KeyExchangeAlgorithm;

use super::inputs::InputInfo;

const MAX_RUNS: usize = 10_000;
const MIN_RUNS: usize = 5;
const TIMEOUT: Duration = Duration::from_secs(1);

// ew
type SolutionFunc = dyn Fn() -> Box<dyn ToString>;
type NanoSec = u128;

pub struct ExecutionResults {
    print: HashSet<String>,
    times_ns: Vec<NanoSec>,
}
impl ExecutionResults {
    pub fn run(func: &SolutionFunc, execution: &ExecutionType) -> Self {
        let mut times_ns = Vec::new();
        let start_time = Instant::now();
        let mut function_results = HashSet::new();

        let max_runs = match execution {
            ExecutionType::Run => 1,
            ExecutionType::Bench => MAX_RUNS,
            ExecutionType::Helping => panic!("why are we running"),
        };

        for r in 0..max_runs {
            if start_time.elapsed() >= TIMEOUT && r > MIN_RUNS {
                break;
            }

            let run_start = Instant::now();
            let result_new = func();
            times_ns.push(run_start.elapsed().as_nanos() as NanoSec);

            function_results.insert(result_new.to_string());
        }
        times_ns.sort_unstable();
        Self {
            print: function_results,
            times_ns,
        }
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
        let error = self
            .times_ns
            .iter()
            .map(|t| mean.abs_diff(*t).pow(2))
            .sum::<u128>() as f32;
        let correction = -1.5 + 1.0 / (8.0 * (n - 1.0));
        (error / (n + correction)).sqrt()
    }
    pub fn is_deterministic(&self) -> bool {
        self.print.len() <= 1
    }
    pub fn get_results(&self) -> String {
        if self.is_deterministic() {
            return self.print.iter().next().unwrap().clone()
        }
        let possible = self.print.iter().cloned().collect::<Vec<_>>().join(", ");
        format!("{{{}}}", possible)
    }
}
pub struct DaySolution {
    part1: Option<Box<SolutionFunc>>,
    part2: Option<Box<SolutionFunc>>,
    date: InputInfo,
}

pub struct ExecuteAllSolutions {
    days: Vec<DaySolution>,
    method: ExecutionType,
}
impl ExecuteAllSolutions {
    pub fn new(items: Vec<DaySolution>, method: ExecutionType) -> Self {
        Self { days: items, method }
    }
    pub fn run(mut self) {
        self.days
            .sort_by(|a, b| a.date.year.cmp(&b.date.year).then(a.date.day.cmp(&b.date.day)));

        match self.method {
            ExecutionType::Run => self.run_bland_verbose(),
            ExecutionType::Bench => self.run_pretty(),
            ExecutionType::Helping => panic!("why are we even here"),
        }
    }
    // https://www.reddit.com/r/adventofcode/comments/1hlyocd/500_in_less_than_a_second/
    pub fn run_bland_verbose(self) {
        let mut total_time = 0;
        for item in self.days {
            println!("\x1b[33m{} Day {:>8}:\x1b[0m", item.date.year, item.date.day);
            print!("Part 1: ");
            if let Some(part1) = item.part1 {
                let exec = ExecutionResults::run(&part1, &self.method);
                total_time += exec.mean();
                println!("{}", exec.get_results())
            } else {
                println!("n/a")
            }
            print!("Part 2: ");
            if let Some(part2) = item.part2 {
                let exec = ExecutionResults::run(&part2, &self.method);
                total_time += exec.mean();
                println!("{}", exec.get_results())
            }
        }
        println!("Total time: {}", total_time);
    }
    // https://www.reddit.com/r/adventofcode/comments/1hlzvvr/10_years_thank_you_eric/
    pub fn run_pretty(self) {
        assert!(self.method != ExecutionType::Helping);
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ExecutionType {
    Run,
    Bench,
    Helping,
}



// 

