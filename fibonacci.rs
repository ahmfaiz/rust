fn main() {
    let mut num_1: u32 = 0;
    let mut num_2: u32 = 1;
    let mut sum: u32;

    let range: u32 = 10;

    println!("0");
    println!("1");

    for _ in 0..range-2 {
        sum = num_1 + num_2;
        println!("{sum}");
        num_1 = num_2;
        num_2 = sum;
    }
}