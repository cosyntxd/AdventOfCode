use std::fs::File;

pub fn run() -> u32 {
    let input = std::fs::read_to_string("data/day01.txt").unwrap().trim().replace('\r', "");;
    let (mut a, mut b) = (vec![], vec![]);
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        a.push(parts.next().unwrap().parse::<u32>().unwrap());
        b.push(parts.next().unwrap().parse::<u32>().unwrap());
    }
    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(lhs, rhs)| u32::abs_diff(lhs, rhs)).sum()

}