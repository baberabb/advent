use anyhow::Result;
use std::fs::read_to_string;

pub struct Day2Lists {
    pub list: Vec<u32>,
}

pub fn check_increasing_decreasing(numbers: &[u32]) -> bool {
    let valid = numbers
        .windows(2)
        .all(|w| w[0].abs_diff(w[1]) > 0 && w[0].abs_diff(w[1]) <= 3)
        && (numbers.windows(2).all(|w| w[0] < w[1]) || numbers.windows(2).all(|w| w[0] > w[1]));

    valid
}
pub fn read_lines_2(path: &str) -> Result<u32> {
    let mut total = 0;
    for line in read_to_string(path)?.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let is_safe = check_increasing_decreasing(&numbers);
        if is_safe {
            total += 1;
        } else {
            for i in 0..numbers.len() {
                let mut next_line = numbers.clone();
                next_line.remove(i);
                let is_safe = check_increasing_decreasing(&next_line);
                if is_safe {
                    total += 1;
                    break;
                }
            }
        }
    }
    Ok(total)
}

pub async fn day2_res(path: &str) {
    let res = read_lines_2(path);
    println!("{:#?}", res);
}
