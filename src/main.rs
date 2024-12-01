use advent::day1::{part1, part2};
use advent::read_lists;
use tokio;
#[tokio::main]
async fn main() {
    let res = read_lists("/Users/baber/Downloads/input.txt").unwrap();
    let res_clone = res.clone();
    let handle1 = tokio::spawn(tokio::task::spawn_blocking(move || part1(&res)));
    let handle2 = tokio::spawn(tokio::task::spawn_blocking(move || part2(&res_clone)));

    let (part1_, part2_) = tokio::join!(handle1, handle2);

    println!("{:?}", part1_.unwrap().unwrap());
    println!("{:?}", part2_.unwrap().unwrap());
}
