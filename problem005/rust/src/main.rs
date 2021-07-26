// use num::integer::gcd;
use num::integer::lcm;
fn main() {
    println!("Problem 005");
    let mut answer: i64 = 1;
    for i in 2..20 {
        answer = lcm(answer, i);
        println!("{}", answer); 
    }
}
