use itertools::Itertools;

pub fn run(input: &String) -> (i32, i32) {
    let input_iter = input.lines().collect::<Vec<&str>>();
    let mut orderings = vec![];
    for line in &input_iter {
        if line.trim().len() == 0 {
            break
        };
        let parsed = line.trim().split("|").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        orderings.push((parsed[0], parsed[1]));
    }
    let mut sum = 0;
    let mut sum2 = 0;
    let mut bads = vec![];

    for line in input_iter.iter().skip(orderings.len() +1) {
        let mut isgood = true;

        let nums = line.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        

        for &(a, b) in &orderings {
            let index1 = nums.iter().position(|&x| x == a);
            let index2 = nums.iter().position(|&x| x == b);

            if let (Some(i1), Some(i2)) = (index1, index2) {
                if i1 > i2 {
                    isgood = false;
                    break;
                }
            }
        }

        if isgood {
            sum += nums[nums.len() / 2];
        } else {
            bads.push(nums.clone());
        }
    }

    for mut trial in bads {
        while !(is_sorted(&trial, &orderings)) {
            trial.sort_by(|a, b| {
                if orderings.iter().any(|&(x, y)| x == *a && y == *b) {
                    std::cmp::Ordering::Less
                } else if orderings.iter().any(|&(x, y)| x == *b && y == *a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }

        if is_sorted(&trial, &orderings) {
            sum2 += trial[trial.len() / 2];
            continue
        }
    }
    (sum as i32, sum2)
}

fn is_sorted(nums: &Vec<i32>, orderings: &Vec<(i32, i32)>) -> bool {
    for &(a, b) in orderings {
        let index1 = nums.iter().position(|&x| x == a);
        let index2 = nums.iter().position(|&x| x == b);

        if let (Some(i1), Some(i2)) = (index1, index2) {
            if i1 > i2 {
                return false
            }
        }
    }
    return true;
}