use std::time::Instant;

use solutions::*;
mod solutions;

fn main() {
    let start = Instant::now();
    day01::run();
    let duration = start.elapsed();
    println!("Done! 🎉 -- Elapsed time: {:?}", duration);

}