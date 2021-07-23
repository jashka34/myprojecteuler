fn main() {
    println!("Problem 1");
    let mut sum: i32 = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
            println!("{}", n);
        }
    }
    println!("{}", sum);
}
