use sortsearchlib::*;
use std::time::{Duration, Instant};

fn main() {
    let mut list = vec![10, -1, 13, 8, 2, 4];
    println!("Before {:?}", &list);
    let start = Instant::now();
    BubbleSort::sort(&mut list).expect("Failed");
    let duration = start.elapsed();
    println!("{:?}", duration);
    println!("After {:?}", &list);
}