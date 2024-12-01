use super::Lists;
use anyhow::Result;
use std::collections::HashMap;

pub fn part1(input: &Lists) -> Result<u32> {
    let mut list1: Vec<u32> = input.list1.clone();
    let mut list2: Vec<u32> = input.list2.clone();

    list1.sort();
    list2.sort();

    let res = list1
        .iter()
        .zip(list2.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum();
    Ok(res)
}

pub fn part2(input: &Lists) -> Result<u32> {
    let counter: HashMap<u32, u32> = input.list2.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    let mut res = 0;
    for i in input.list1.iter() {
        res += i * counter.get(&i).unwrap_or(&0)
    }
    Ok(res)
}
