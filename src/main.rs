use advent::day3::read_day3;

#[tokio::main]
async fn main() {
    let res = read_day3("/Users/baber/Downloads/day3.txt").expect("TODO: panic message");
    println!("{:?}", res);
}
