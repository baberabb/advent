use advent::day2::day2_res;

#[tokio::main]
async fn main() {
    day2_res("/Users/baber/Downloads/day2.txt")
        .await
        .expect("TODO: panic message");
}
