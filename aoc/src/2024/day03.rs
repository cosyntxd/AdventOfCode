use regex::Regex;

fn mul(n: &str) -> u32 {
    let str = n.replace("mul(", "").replace(")", "");
    let (a,b) = str.split_at(str.find(',').unwrap());
    return a.parse::<u32>().unwrap() * b.replace(",", "").parse::<u32>().unwrap();
}

pub fn run(input: &String) -> (u32, u32) {

    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total_sum = 0;
    for caps in pattern.find_iter(&input) {
        total_sum += mul(caps.as_str());
    }

    let pattern2 = Regex::new(r"mul\(\d+,\d+\)|do(n't)?\(\)").unwrap();
    let mut exec = true;
    let mut total_2 = 0;
    for caps in pattern2.find_iter(&input) {
        match caps.as_str() {
            "do()" => {exec = true},
            "don't()" => {exec = false},
            _ => {
                if exec {
                    total_2 += mul(caps.as_str());
                }
            }   
        }
    }

    (total_sum as u32, total_2 as u32)
}