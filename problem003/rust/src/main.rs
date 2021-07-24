use rust::prime::is_prime;

fn main() {
    let n: u64 = 600851475143;
    //let n: u64   = 99111475143;
    let sq = (n as f64).sqrt();
    let maxi: u64 = (sq as u64) + 1;
    let mut max: u64 = 0;
    println!("maxi={}", maxi);
    for i in 1..maxi {
        if n % i == 0 {
           //println!("{} is prime!", i);
           if is_prime(i) {
               max = i;
            }
        }
    }
    println!("max={}",max);
}
