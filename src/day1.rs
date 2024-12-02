use super::{read_lists, Day1Lists};
use anyhow::Result;
use std::collections::HashMap;

pub fn part1(input: &Day1Lists) -> Result<u32> {
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

pub fn part2(input: &Day1Lists) -> Result<u32> {
    let counter: HashMap<u32, u32> = input.list2.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    let mut res = 0;
    for i in input.list1.iter() {
        res += i * counter.get(i).unwrap_or(&0)
    }
    Ok(res)
}

async fn day1_res(path: &str) -> Result<()> {
    let res = read_lists(path)?;
    let res_clone = res.clone();
    let handle1 = tokio::spawn(tokio::task::spawn_blocking(move || part1(&res)));
    let handle2 = tokio::spawn(tokio::task::spawn_blocking(move || part2(&res_clone)));

    let (part1_, part2_) = tokio::join!(handle1, handle2);

    println!("{:?}", part1_.unwrap()?);
    println!("{:?}", part2_.unwrap()?);
    Ok(())
}
