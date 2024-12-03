fn is_safe(level: &Vec<i32>) -> bool {
    let a = level.windows(2).all(|w| w[0] != w[1]) && 
                       (level.windows(2).all(|w| w[0] < w[1]) || 
                        level.windows(2).all(|w| w[0] > w[1]));

    let b = level.windows(2).all(|w| (w[0] - w[1]).abs().clamp(1, 3) == (w[0] - w[1]).abs());

    a && b
}

fn safeWR(level: &Vec<i32>) -> bool {
    if (is_safe(level)) {
        return true;
    }
    for i in 0..level.len() {
        let mut clone = level.clone();
        clone.remove(i);

        if is_safe(&clone) {
            return true
        }
    }

    false
}

pub fn run() -> (u32, u32) {
    let input = std::fs::read_to_string("data/day02.txt").unwrap().trim().replace('\r', "");
    let mut a1 = 0;
    let mut a2 = 0;
    for line in input.lines() {
        let row = line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        if is_safe(&row) {
            a1 += 1;
        }
        if safeWR(&row) {
            a2 += 1;
        }
    }
    (a1, a2)
    
}
