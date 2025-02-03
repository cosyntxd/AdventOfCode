// 
pub fn run(input: &String) -> (u32, u32) {
    let (mut a, mut b) = (vec![], vec![]);
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        a.push(parts.next().unwrap().parse::<u32>().unwrap());
        b.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    let answer2 = a.iter().map(|x| x * b.iter().filter(|y| x == *y).count() as u32).sum();

    a.sort();
    b.sort();

    let answer1 = a.into_iter().zip(b).map(|(lhs, rhs)| u32::abs_diff(lhs, rhs)).sum();
    (answer1, answer2)
}

