use rust::prime::is_prime;

fn main() {
    let n: u64 = 600851475143;
    //let n: u64   = 99111475143;
    let sq = (n as f64).sqrt();
    let maxi: u64 = (sq as u64) + 1;
    let mut max: u64 = 0;
    println!("maxi={}", maxi);
    for i in 1..maxi {
        if is_prime(i) {
           //println!("{} is prime!", i);
           if n % i == 0 {
               max = i;
            }
        }
    }
    println!("max={}",max);
}
