fn main() {
    println!("Problem 006");
    let mut sum100: u32 = 0;
    let mut sumq100: u32 = 0;
    for i in 1..101 {
        // println!("{}", i);
        sum100 += i;
        sumq100 += i * i;
    }
    println!("{}", sum100 * sum100 - sumq100);
}
