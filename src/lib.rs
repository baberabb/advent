use std::fs::read_to_string;
pub mod day1;
pub mod day2;
pub mod day3;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Clone)]
pub struct Day1Lists {
    pub list1: Vec<u32>,
    pub list2: Vec<u32>,
}

pub fn read_lists(path: &str) -> anyhow::Result<Day1Lists> {
    let mut line1 = Vec::new();
    let mut line2 = Vec::new();

    for line in read_to_string(path)?.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        line1.push(numbers[0]);
        line2.push(numbers[1]);
    }
    Ok(Day1Lists {
        list1: line1,
        list2: line2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
