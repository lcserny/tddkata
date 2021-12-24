use std::time::Instant;
use tddkata::strings::add;

fn main() {
    let start = Instant::now();
    add("//###\n6###6").unwrap();
    let time = start.elapsed();
    println!("Time to add complex: {:?}", time);
}
