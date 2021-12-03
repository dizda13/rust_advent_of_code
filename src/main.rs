pub mod day1;
pub mod day2;
pub mod shared;

#[tokio::main]
async fn main() { 
    println!("Day one part 1 - result {}", day1::part1().await);
    println!("Day one part 2 - result {}", day1::part2().await);
}
