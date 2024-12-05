use std::{thread, time::Instant};

use solutions::*;
mod solutions;

fn main() {
    let start = Instant::now();
    // println!("{:?}", day01::run());
    // println!("{:?}", day02::run());
    // println!("{:?}", day03::run());
    // println!("{:?}", day04::run());
    println!("{:?}", day05::run());
    let duration = start.elapsed();
    println!("Done! 🎉 -- Elapsed time: {:?}", duration);

}